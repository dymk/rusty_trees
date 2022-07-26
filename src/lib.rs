mod binary_tree;
pub mod file_tree;
mod iter;
mod node_path;
mod trie;
#[macro_use]
mod macro_impl;

pub use node_path::{IntoComponents, NodePath};
pub use trie::Trie;

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use itertools::assert_equal;

    use crate::{NodePath, Trie};

    #[test]
    fn test_works() {
        let trie = trie![
            "do", 1;
            trie!['g', 2],
            trie!["ts", 3],
        ];

        println!("trie:\n{:?}", trie);
        assert_trie_iter(vec![("do", 1), ("dog", 2), ("dots", 3)], &trie);
        assert_eq!(Some(&3), trie.get("dots"));
    }

    fn assert_trie_iter<P1, P2, V>(expect: Vec<(P1, V)>, trie: &Trie<P2, V>)
    where
        P1: Into<P2> + Copy,
        P2: NodePath + Eq + Debug,
        V: Eq + Debug,
    {
        assert_equal(expect.iter().map(|(c, i)| ((*c).into(), i)), trie.iter());
    }
}
