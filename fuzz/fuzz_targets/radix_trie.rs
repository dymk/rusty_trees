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
    if let Ok(data) = std::str::from_utf8(data) {
        // only clear ascii
        if !data.is_ascii() {
            return;
        }

        let mut trie: RadixTrie<String, ()> = RadixTrie::default();
        let mut truth = HashMap::new();

        for line in data.lines() {
            if line.len() < 2 {
                return;
            }

            let rest = &line[2..];
            match &line[..2] {
                "i:" => {
                    check_same!(trie, truth, "insert", insert(rest.to_owned(), ()));
                }
                "g:" => {
                    check_same!(trie, truth, "get_ref", get(rest));
                }
                "G:" => {
                    check_same!(trie, truth, "get_owned", get(&rest.to_owned()));
                }
                "r:" => {
                    check_same!(trie, truth, "remove_ref", remove(rest));
                }
                _ => {
                    return;
                }
            }
        }
    }
});
