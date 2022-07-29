use super::RadixTrie;
use std::fmt::Debug;

impl<P, V> Debug for RadixTrie<P, V>
where
    P: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        for node in &trie.nodes {
            f.write_str(&format!(
                "{}- {:?} ({:?})\n",
                ident_str, node.path, node.value
            ))?;
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
            nodes: vec![Node {
                path: "foo".into(),
                value: Some(5),
                trie: RadixTrie::default(),
            }],
        };

        format!("{:?}", trie);
    }
}
