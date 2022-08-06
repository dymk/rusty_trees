#![no_main]
use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use trees::radix_trie::RadixTrie;

macro_rules! check_same {
    ($trie:ident, $truth:ident, $name:expr, $($lambda:tt)*) => {
        let a = $trie.$($lambda)*;
        let b = $truth.$($lambda)*;
        if a != b {
            panic!("method {} failed: {:?} != {:?}", $name, a, b);
        }
    };
}

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        // only clear ascii
        if !data.is_ascii() {
            return;
        }

        let mut trie = RadixTrie::new();
        let mut truth = HashMap::new();

        for line in data.lines() {
            if line.len() < 2 {
                return;
            }

            let rest = &line[2..];
            match &line[..2] {
                "i:" => {
                    let (param, rest) = get_param(rest).unwrap_or((0, rest));
                    check_same!(trie, truth, "insert", insert(rest.to_owned(), param));
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

        let mut iter_truth = HashMap::new();
        for (path, value) in trie.iter() {
            if let Some(old_value) = iter_truth.insert(path.clone(), *value) {
                panic!(
                    "iterator gave duplicate path: {} / {} / {}",
                    path, value, old_value
                );
            }
        }
        if !iter_truth.eq(&truth) {
            panic!("iterators not equal: {:?} / {:?}", iter_truth, truth);
        }
    }

    fn get_param(line: &str) -> Option<(usize, &str)> {
        let idx = line.find(",")?;
        let (line, num) = line.split_at(idx);
        let num = num[1..].parse().ok()?;
        Some((num, line))
    }
});
