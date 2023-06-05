#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

/// Main struct containing the underlying `HashMap`.
#[derive(Serialize, Deserialize)]
pub struct MarkovChain {
    order: usize,
    chain: HashMap<Vec<String>, Vec<(String, u32)>>,
}

impl MarkovChain {
    /// Creates a new chain with the specified order.
    ///
    /// # Panics
    ///
    /// Panics if the `order` is 0.
    #[must_use]
    pub fn new(order: usize) -> Self {
        assert_ne!(order, 0, "order cannot be 0");

        Self {
            order,
            chain: HashMap::new(),
        }
    }

    /// Learns new words and their relations. The input should have at least
    /// `order` words for this function to do anything.
    pub fn train(&mut self, mut input: impl Iterator<Item = String>) {
        let mut previous_words = input
            .by_ref()
            .take(self.order)
            .map(Into::into)
            .collect::<VecDeque<_>>();

        for word in input {
            let pair = (word.clone(), 1);
            match self.chain.get_mut(&Vec::from(previous_words.clone())) {
                Some(pairs) => match pairs.iter_mut().find(|pair| pair.0 == word) {
                    Some(pair) => pair.1 += 1,
                    None => {
                        pairs.push(pair);
                    }
                },
                None => {
                    self.chain.insert(previous_words.clone().into(), vec![pair]);
                }
            }

            previous_words.pop_front();
            previous_words.push_back(word);
        }
    }

    /// Generates text using learned word relations.
    ///
    /// Returns `None` when the chain is empty.
    #[allow(clippy::missing_panics_doc)] // shouldn't panic
    #[must_use]
    pub fn generate_text(&self, max_len: u32) -> Option<String> {
        let mut rng = rand::thread_rng();
        let mut words = self.chain.keys().choose(&mut rng)?.clone();

        for _ in 0..max_len {
            let word = match self.chain.get(&words[words.len() - self.order..].to_vec()) {
                Some(pairs) => {
                    let dist = WeightedIndex::new(pairs.iter().map(|pair| pair.1)).unwrap();
                    pairs[dist.sample(&mut rng)].0.clone()
                }
                None => break,
            };

            words.push(word.clone());
        }

        Some(words.join(" "))
    }

    /// Returns this chain's size (the amount of `order`-sized word sets).
    #[must_use]
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Returns `true` if the chain is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_chain() {
        let chain = MarkovChain::new(1);

        assert_eq!(chain.len(), 0);
        assert!(chain.generate_text(16).is_none());
    }

    #[test]
    fn test_chain() {
        let mut chain = MarkovChain::new(2);

        chain.train(
            "lorem ipsum dolor sit amet"
                .split_ascii_whitespace()
                .map(Into::into),
        );

        assert_eq!(chain.len(), 3);
        assert!(chain
            .generate_text(16)
            .is_some_and(|text| text.ends_with("dolor sit amet")));
    }

    #[test]
    fn test_text_generation() {
        let mut chain = MarkovChain::new(1);

        chain.train("a a a a".split_ascii_whitespace().map(Into::into));

        assert_eq!(chain.len(), 1);
        assert_eq!(chain.generate_text(1).unwrap(), "a a");
        assert_eq!(chain.generate_text(2).unwrap(), "a a a");
        assert_eq!(chain.generate_text(3).unwrap(), "a a a a");
    }
}
