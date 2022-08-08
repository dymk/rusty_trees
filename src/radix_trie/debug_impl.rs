use super::RadixTrie;
use std::fmt::Debug;

impl<P, V> Debug for RadixTrie<P, V>
where
    P: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(root) ")?;
        Self::fmt_impl(0, self, f)?;
        Ok(())
    }
}

impl<P, V> RadixTrie<P, V>
where
    P: Debug,
    V: Debug,
{
    fn fmt_impl(ident: usize, trie: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ident_str: String = " ".repeat(ident);
        f.write_str(&format!("`{:?}`\n", trie.value))?;
        for node in &trie.nodes {
            f.write_str(&format!("{}- {:?} ", ident_str, node.key))?;
            RadixTrie::fmt_impl(ident + 2, &node.trie, f)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::radix_trie::{Node, RadixTrie};

    #[test]
    fn test_can_print_debug() {
        let trie: RadixTrie<String, i32> = RadixTrie {
            value: Some(5),
            nodes: vec![Node {
                key: "foo".into(),
                trie: RadixTrie::default(),
            }],
        };

        format!("{:?}", trie);
    }
}
