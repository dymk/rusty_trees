Rusty Trees
====

A place to put various tree data structures written in Rust.

See the [documentation here](https://dymk.github.io/rusty_trees/docs/rusty_trees/)

Currently implemented:
- [`RadixTrie`](https://dymk.github.io/rusty_trees/docs/rusty_trees/radix_trie/struct.RadixTrie.html) (also known as a [compressed trie or compact prefix trie](https://en.wikipedia.org/wiki/Radix_tree))
- Methods (that behave the same as `HashMap`):
  - `insert`
  - `remove`
  - `get`
  - `iter`
  - `iter_mut`


The API for `RadixTrie` is similar to `HashMap`. See [the documentation](https://dymk.github.io/rusty_trees/docs/rusty_trees/radix_trie/struct.RadixTrie.html) for more info.

`RadixTrie` has a fuzzing harness built with [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz). See `fuzz/fuzz_targets/radix_trie.rs` for the implementation.

Run the fuzzer with the following:
```
$> cd fuzz
$> cargo install cargo-fuzz # if not already installed
$> cargo +nightly fuzz run radix_trie
$> cargo +nightly fuzz run -j8 radix_trie # run multiple fuzzer workers
```
