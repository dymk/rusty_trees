use std::fs;
use std::path::Path;

fn main() {
    let dest_path = Path::new("src/radix_trie/fuzzer_tests.rs");
    if !dest_path.exists() {
        fs::write(
            &dest_path,
            r#"
        // this is an automatically generated file (see build.rs)
        // run ./fuzz/corpus_to_generated_tests after running the fuzzer
        // to populate it
        "#,
        )
        .unwrap();
    }
    println!("cargo:rerun-if-changed=src/radix_trie/fuzzer_tests.rs");
}
