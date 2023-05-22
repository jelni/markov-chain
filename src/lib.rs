#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MarkovChain {
    order: usize,
    chain: HashMap<VecDeque<String>, Vec<(String, u32)>>,
}

impl MarkovChain {
    #[must_use]
    pub fn new(order: usize) -> Self {
        Self {
            order,
            chain: HashMap::new(),
        }
    }

    pub fn save(&self, writer: &mut impl Write) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(writer, &self)
    }

    pub fn load(reader: impl Read) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(reader)
    }

    pub fn train(&mut self, input: &str) {
        let mut input = input.split_ascii_whitespace();

        let mut previous_words = input
            .by_ref()
            .take(self.order)
            .map(Into::into)
            .collect::<VecDeque<_>>();

        for word in input {
            let pair = (word.to_owned(), 1);
            match self.chain.get_mut(&previous_words) {
                Some(pairs) => match pairs.iter_mut().find(|pair| pair.0 == word) {
                    Some(pair) => pair.1 += 1,
                    None => {
                        pairs.push(pair);
                    }
                },
                None => {
                    self.chain.insert(previous_words.clone(), vec![pair]);
                }
            }

            previous_words.pop_front();
            previous_words.push_back(word.into());
        }
    }

    #[must_use]
    pub fn generate_text(&self, max_len: u32) -> Option<String> {
        let mut rng = rand::thread_rng();
        let mut words = Vec::from(self.chain.keys().choose(&mut rng)?.clone());

        for _ in 0..=max_len {
            let word = match self
                .chain
                .get(&words[words.len() - self.order..].iter().cloned().collect())
            {
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

    #[must_use]
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
