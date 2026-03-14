use ndarray::{Array2, Axis};
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use std::collections::{HashMap, HashSet};

pub struct Markov {
    pub matrix: Array2<f64>,
    pub char_to_idx: HashMap<char, usize>,
    pub idx_to_char: Vec<char>,
}

impl Markov {
    pub fn train(names: &[&str]) -> Self {
        let mut charset: HashSet<char> = HashSet::new();

        for name in names {
            for c in name.chars() {
                charset.insert(c);
            }
        }

        charset.insert('^'); // start
        charset.insert('$'); // end

        let idx_to_char: Vec<char> = charset.into_iter().collect();

        let mut char_to_idx = HashMap::new();
        for (i, c) in idx_to_char.iter().enumerate() {
            char_to_idx.insert(*c, i);
        }

        let n = idx_to_char.len();

        let mut matrix = Array2::<f64>::zeros((n, n));

        for name in names {
            let mut prev = '^';

            for c in name.chars() {
                let i = char_to_idx[&prev];
                let j = char_to_idx[&c];

                matrix[[i, j]] += 1.0;

                prev = c;
            }

            let i = char_to_idx[&prev];
            let j = char_to_idx[&'$'];

            matrix[[i, j]] += 1.0;
        }

        // normalize probabilities
        for mut row in matrix.axis_iter_mut(Axis(0)) {
            let sum: f64 = row.sum();
            if sum > 0.0 {
                row /= sum;
            }
        }

        Self {
            matrix,
            char_to_idx,
            idx_to_char,
        }
    }

    pub fn generate(&self, rng: &mut impl Rng) -> String {
        let mut result = String::new();

        let mut current = self.char_to_idx[&'^'];

        loop {
            let probs = self.matrix.row(current);

            let dist = WeightedIndex::new(probs.to_vec()).unwrap();

            let next = dist.sample(rng);

            let c = self.idx_to_char[next];

            if c == '$' {
                break;
            }

            result.push(c);

            current = next;
        }

        result
    }
}

