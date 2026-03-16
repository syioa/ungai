use std::collections::HashMap;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub struct Markov {
    pub transitions: HashMap<(u8, u8), HashMap<u8, f64>>,
}

impl Markov {
    pub fn train(names: &[&str], smoothing: f64) -> Self {
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
                if current == b'$' { break; }
            }
        }
        Self { transitions }
    }

    pub fn generate(&self, rng: &mut impl rand::Rng) -> String {
        let mut result = String::new();
        let mut p1 = b'^';
        let mut p2 = b'^';

        loop {
            let next_map = match self.transitions.get(&(p1, p2)) {
                Some(map) => map,
                None => break,
            };

            // Convert frequency map to WeightedIndex
            let choices: Vec<u8> = next_map.keys().cloned().collect();
            let weights: Vec<f64> = next_map.values().cloned().collect();
            
            let dist = WeightedIndex::new(&weights).unwrap();
            let next = choices[dist.sample(rng)];

            if next == b'$' { break; }

            result.push(next as char);
            p1 = p2;
            p2 = next;
        }
        result
    }
}
