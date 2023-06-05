# markov-chain

A simple [Markov chain](https://wikipedia.org/wiki/Markov_chain) Rust
implementation I made to better understand this data structure.

Any order (HashMap's key length) can be used. This decides how many words are
taken into account when generating text. More will result in better generation
results, but also requires more training input.

It allows saving and restoring the state from any format that
[Serde](https://crates.io/crates/serde)) supports.

## Example

```rs
fn main() {
    let mut chain = MarkovChain::new(2);

    chain.train("lorem ipsum dolor sit amet");

    // smart ai bot 🤯
    chain.generate_text(16); // lorem ipsum dolor sit amet

    chain.save(&mut BufWriter::new(File::create("model.dat").unwrap())).unwrap();

    let chain = MarkovChain::load(BufReader::new(File::open("model.dat").unwrap())).unwrap();

    println!("loaded {} entries", chain.len()); // loaded 3 entries
}
```
