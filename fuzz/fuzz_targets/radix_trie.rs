#![no_main]

extern crate arbitrary;

use std::collections::HashMap;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use rusty_trees::radix_trie::RadixTrie;

macro_rules! check_same {
    ($trie:ident, $truth:ident, $name:expr, $($lambda:tt)*) => {
        let a = $trie.$($lambda)*;
        let b = $truth.$($lambda)*;
        if a != b {
            panic!("method {} failed: {:?} != {:?}", $name, a, b);
        }
    };
}

#[derive(Arbitrary)]
enum Action {
    Insert(String, usize),
    Get(String),
    Remove(String),
}

static mut PRINTED: bool = false;

fuzz_target!(|data: &[u8]| {
    let mut source = Unstructured::new(data);
    if let Ok(actions) = source.arbitrary::<Vec<Action>>() {
        #[cfg(feature = "print_test_body")]
        {
            if unsafe { !PRINTED } {
                unsafe { PRINTED = true };
                print_test_body(actions);
            }
        }

        #[cfg(not(feature = "print_test_body"))]
        {
            let mut truth: HashMap<String, usize> = HashMap::new();
            let mut trie = RadixTrie::new();
            run_testcase(actions, &mut trie, &mut truth);
        }
    }
});

#[allow(dead_code)]
fn print_test_body(actions: Vec<Action>) {
    let print_assert_eq = |expected: String, actual: String| {
        println!("\tassert_eq!(\n\t\t{}, \n\t\t{}\n\t);", expected, actual);
    };

    println!("\t#[allow(unused_mut)]");
    println!("\tlet mut trie: RadixTrie<String, usize> = RadixTrie::new();");
    println!("\t#[allow(unused_mut)]");
    println!("\tlet mut truth: HashMap<String, usize> = HashMap::new();");

    for action in actions {
        match action {
            Action::Insert(key, param) => {
                print_assert_eq(
                    format!("truth.insert({:?}.to_owned(), {:?})", key, param),
                    format!("trie.insert({:?}.to_owned(), {:?})", key, param),
                );
            }
            Action::Get(key) => {
                print_assert_eq(
                    format!("truth.get({:?})", key),
                    format!("trie.get({:?})", key),
                );
            }
            Action::Remove(key) => {
                print_assert_eq(
                    format!("truth.remove({:?})", key),
                    format!("trie.remove({:?})", key),
                );
            }
        };
        println!("");
    }
    println!("\tformat!(\"{{:?}}\", trie);");
    println!("\ttest_iterators(truth, trie);");
}

#[allow(dead_code)]
fn run_testcase(
    actions: Vec<Action>,
    trie: &mut RadixTrie<String, usize>,
    truth: &mut HashMap<String, usize>,
) {
    for action in actions {
        match action {
            Action::Insert(key, param) => {
                check_same!(trie, truth, "insert", insert(key.clone(), param));
            }
            Action::Get(key) => {
                check_same!(trie, truth, "get_ref", get(&key));
            }
            Action::Remove(key) => {
                check_same!(trie, truth, "remove_ref", remove(&key));
            }
        }
    }

    let mut iter_truth = HashMap::new();
    for (key, value) in trie.iter() {
        if let Some(old_value) = iter_truth.insert(key.clone(), *value) {
            panic!(
                "iterator gave duplicate key: {} / {} / {}",
                key, value, old_value
            );
        }
    }
    if !iter_truth.eq(&truth) {
        panic!("iterators not equal: {:?} / {:?}", iter_truth, truth);
    }
}
