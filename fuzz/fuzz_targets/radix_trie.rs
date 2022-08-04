#![no_main]
use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use trees::radix_trie::RadixTrie;

macro_rules! check_same {
    ($trie:ident, $truth:ident, $name:expr, $($lambda:tt)*) => {
        let a = $trie.$($lambda)*;
        let b = $truth.$($lambda)*;
        if a != b {
            panic!("{}: {:?} != {:?}", $name, a, b);
        }
    };
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if !s.is_ascii() {
            return;
        }
        let mut trie: RadixTrie<String, ()> = RadixTrie::default();
        let mut truth = HashMap::new();

        for line in s.lines() {
            if line.len() == 0 {
                continue;
            }

            match line.chars().next().unwrap() {
                'a' => {
                    check_same!(trie, truth, "insert", insert(line[1..].to_owned(), ()));
                }
                'g' => {
                    check_same!(trie, truth, "get_ref", get(&line[1..]));
                }
                'G' => {
                    check_same!(trie, truth, "get_owned", get(&(line[1..].to_owned())));
                }
                _ => {
                    continue;
                }
            }
        }
    }
});
