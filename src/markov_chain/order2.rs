use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Write};

#[derive(Serialize, Deserialize)]
pub struct Markov {
    pub transitions: HashMap<(u8, u8), HashMap<u8, f64>>,
}

impl Markov {
    pub fn train(names: &[String], smoothing: f64) -> Self {
        let mut transitions: HashMap<(u8, u8), HashMap<u8, f64>> = HashMap::new();

        for name in names {
            let bytes = name.as_bytes();
            // Start with double markers
            let mut p1 = b'^';
            let mut p2 = b'^';

            for &current in bytes.iter().chain(std::iter::once(&b'$')) {
                transitions
                    .entry((p1, p2))
                    .or_insert_with(|| {
                        let mut m = HashMap::new();
                        for c in b'a'..=b'z' {
                            m.insert(c, smoothing);
                        }
                        m
                    })
                    .entry(current)
                    .and_modify(|count| *count += 1.0)
                    .or_insert(1.0 + smoothing);

                p1 = p2;
                p2 = current;
                if current == b'$' {
                    break;
                }
            }
        }
        Self { transitions }
    }

    pub fn precompute_distributions(&self) -> HashMap<(u8, u8), (Vec<u8>, WeightedIndex<f64>)> {
        let mut distributions = HashMap::new();

        for (&state, inner_counts) in &self.transitions {
            let mut choices = Vec::new();
            let mut weights = Vec::new();

            for (&c, &weight) in inner_counts {
                choices.push(c);
                weights.push(weight);
            }

            if let Ok(dist) = WeightedIndex::new(weights) {
                distributions.insert(state, (choices, dist));
            }
        }

        distributions
    }

    pub fn write_transitions_to_file(&self, file_name: &str) -> bincode::Result<()> {
        let bytes = bincode::serialize(&self.transitions)?;

        let compressed = zstd::encode_all(&bytes[..], 3)?;

        let mut file = fs::File::create(file_name)?;
        file.write_all(&compressed)?;
        Ok(())
    }

    pub fn read_transitions_from(file_name: &str) -> bincode::Result<Self> {
        let compressed = fs::read(file_name)?;

        let decompressed = zstd::decode_all(&compressed[..])?;

        let data: Markov = bincode::deserialize(&decompressed)?;
        Ok(data)
    }

    pub fn generate(
        &self,
        rng: &mut impl rand::Rng,
        distributions: &HashMap<(u8, u8), (Vec<u8>, WeightedIndex<f64>)>,
    ) -> String {
        let mut result = String::new();
        let mut p1 = b'^';
        let mut p2 = b'^';

        loop {
            let (choices, dist) = match distributions.get(&(p1, p2)) {
                Some(data) => data,
                None => break,
            };
            let next = choices[dist.sample(rng)];

            if next == b'$' {
                break;
            }

            result.push(next as char);
            p1 = p2;
            p2 = next;
        }
        result
    }
}
