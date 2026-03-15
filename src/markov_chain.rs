use ndarray::{Array2, Axis};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

pub struct Markov {
    pub matrix: Array2<f64>,
}

impl Markov {
    pub fn train(names: &[&str]) -> Self {
        let n = 256;
        let mut matrix = Array2::<f64>::zeros((n, n));

        for name in names {
            let mut prev = b'^' as usize; // start marker

            for c in name.chars() {
                let j = c as usize;
                if j < n {
                    matrix[[prev, j]] += 1.0;
                    prev = j;
                }
            }

            let j = b'$' as usize; // end marker
            matrix[[prev, j]] += 1.0;
        }

        // normalize probabilities
        for mut row in matrix.axis_iter_mut(Axis(0)) {
            let sum: f64 = row.sum();
            if sum > 0.0 {
                row /= sum;
            }
        }

        Self { matrix }
    }

    pub fn generate(&self, rng: &mut impl Rng) -> String {
        let mut result = String::new();
        let mut current = b'^' as usize;

        loop {
            let probs = self.matrix.row(current);
            let dist = WeightedIndex::new(probs.to_vec()).unwrap();
            let next = dist.sample(rng);

            if next == b'$' as usize {
                break;
            }

            result.push(next as u8 as char);
            current = next;
        }

        result
    }
}
