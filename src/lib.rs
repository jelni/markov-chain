#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

/// Main struct containing the underlying `HashMap`.
#[derive(Serialize, Deserialize)]
pub struct MarkovChain {
    order: usize,
    chain: HashMap<VecDeque<String>, Vec<(String, u32)>>,
}

impl MarkovChain {
    /// Creates a new chain with the specified order.
    #[must_use]
    pub fn new(order: usize) -> Self {
        Self {
            order,
            chain: HashMap::new(),
        }
    }

    /// Writes the `MessagePack` serialized struct to the specified writer.
    /// Use this method to store the model on disk.
    ///
    /// # Errors
    ///
    /// This function will return an error if the serialization fails.
    pub fn save(&self, writer: &mut impl Write) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(writer, &self)
    }

    /// Reads the `MessagePack` serialized struct from the specified reader.
    /// Use this method to load the model from disk.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deserialization fails.
    pub fn load(reader: impl Read) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(reader)
    }

    /// Learns new words and their relations. This function splits the input
    /// using [`str::split_ascii_whitespace`]. The input should have at least
    /// `order` words for this function to do anything.
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

    /// Generates text using learned word relations.
    ///
    /// Returns `None` when the chain is empty.
    #[allow(clippy::missing_panics_doc)] // shouldn't panic
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
