# markov-chain

A simple [Markov chain](https://wikipedia.org/wiki/Markov_chain) implementation
in Rust. I made it to better understand this data structure.

Any order (HashMap's key length) can be used. This decides how many words are
taken into account when generating text. More means better generation results,
but also requires more training input.

It supports saving and loading the state to a MessagePack file (and any other
format via [Serde](https://crates.io/crates/serde).

## Example

```rs
fn main() {
    let mut chain = MarkovChain::new(2);

    chain.train("lorem ipsum dolor sit amet");

    // smart ai bot 🤯
    chain.generate_text(16); // lorem ipsum dolor sit amet

    chain.save(&mut BufWriter::new(File::create("model.dat").unwrap())).unwrap();

    let chain = MarkovChain::load(BufReader::new(File::open("model.dat").unwrap())).unwrap();

    println!("loaded {} entries", chain.len());
}
```
