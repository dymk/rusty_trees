# Rusty Trees

A place to put various tree data structures written in Rust. [Rustdocs here](https://dymk.github.io/rusty_trees/docs/rusty_trees/).

## [`RadixTrie<K, V>`](https://dymk.github.io/rusty_trees/docs/rusty_trees/radix_trie/struct.RadixTrie.html)

*(also known as a [compressed trie or compact prefix trie](https://en.wikipedia.org/wiki/Radix_tree))*

A radix trie, `RadixTrie<K, V> where K: Key`, is generic over its key (see trait [`Key`](https://dymk.github.io/rusty_trees/docs/rusty_trees/radix_trie/trait.Key.html)) and value types. There is no restriction on the value type.

An implementation of `Key` for `String` is provided.

### Methods
*Methods behave identically to those of `HashMap`. See [the documentation](https://dymk.github.io/rusty_trees/docs/rusty_trees/radix_trie/struct.RadixTrie.html) for more info.*

- `insert` - insert a value, returning the old value (if any)
- `remove` - remove a value, returning the old value (if any)
- `get` - get the value (if present)
- `iter` - iterate over `(&Key, &Value)` pairs within the trie
- `iter_mut` - mutable iterator over `(&Key, &mut Value)` pairs

## Tests

Run with `cargo test`

## Fuzzing

`RadixTrie` has a fuzzing harness built with [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz). See `fuzz/fuzz_targets/radix_trie.rs` for the implementation.

Run the fuzzer with the following wrapper script, or directly with `cargo fuzz` if building with nightly:
```
$> # if not already installed
$> cargo install cargo-fuzz
$>
$> ./fuzz/fuzz run radix_trie
$> ./fuzz/fuzz run -j8 radix_trie # run multiple fuzzer workers
```

(Note that cargo-fuzz requires a nightly toolchain, and the `./fuzz/fuzz` wrapper script invokes this automatically).

----

Convert the fuzzer corpus into tests to run with `cargo test`, and enable building fuzzer generated tests with the `run_fuzzer_tests` feature:
```
$> ./fuzz/corpus_to_generated_tests
$> cargo test --feature=run_fuzzer_tests
```

To remove the fuzzer generated tests, run `truncate -s0 src/radix_trie/fuzzer_tests.rs`.

---

With the fuzzer corpus included as unit tests, one can use [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) to generate coverage information:

```
$> cargo llvm-cov --verbose --html
$> open target/llvm-cov/html/index.html
```