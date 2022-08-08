use crate::radix_trie::{Key, KeyRef, RadixTrie};

use super::Node;

/// Iterator over a [RadixTrie]
///
/// Yielded items are a tuple of (P, &V), where
/// P is the key type, and V is the value type
pub struct Iter<'a, P, V>(Vec<IterState<'a, P, V>>)
where
    P: Key;

pub struct IterState<'a, P, V>
where
    P: Key,
{
    key: Option<&'a P::Ref>,
    value: Option<&'a V>,
    nodes: Option<&'a [Node<P, V>]>,
}

impl<'a, P, V> Iter<'a, P, V>
where
    P: Key,
{
    pub(super) fn new(tree: &'a RadixTrie<P, V>) -> Self {
        Iter(vec![Self::to_iter_state(None, tree)])
    }

    fn to_iter_state(key: Option<&'a P::Ref>, tree: &'a RadixTrie<P, V>) -> IterState<'a, P, V> {
        IterState {
            key,
            value: tree.value.as_ref(),
            nodes: Some(&tree.nodes[..]),
        }
    }
}

impl<'a, P, V> Iterator for Iter<'a, P, V>
where
    P: Key,
{
    type Item = (P, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.last_mut().and_then(|node_it| node_it.next()) {
                Some(IterStateItem::Value(value)) => {
                    let mut key_iter = self.0.iter().filter_map(|e| e.key);
                    let key = P::Ref::concat(&mut key_iter);
                    return Some((key, value));
                }
                Some(IterStateItem::Trie(key, trie)) => {
                    self.0.push(Self::to_iter_state(Some(key), trie))
                }
                None => {
                    self.0.pop()?;
                }
            }
        }
    }
}

pub enum IterStateItem<'a, P, V>
where
    P: Key,
{
    Value(&'a V),
    Trie(&'a P::Ref, &'a RadixTrie<P, V>),
}

impl<'a, P, V> Iterator for IterState<'a, P, V>
where
    P: Key,
{
    type Item = IterStateItem<'a, P, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(IterStateItem::Value(value));
        }
        let nodes = self.nodes.take()?;
        let (head, rest) = nodes.split_first()?;
        self.nodes = Some(rest);
        return Some(IterStateItem::Trie(head.key.borrow(), &head.trie));
    }
}

#[cfg(test)]
mod test {
    use itertools::assert_equal;

    use crate::radix_trie::RadixTrie;

    #[test]
    fn test_works() {
        let mut trie = RadixTrie::<String, _>::new();
        trie.insert("".into(), 1);
        assert_equal(vec![("".into(), &1)].into_iter(), trie.iter());

        trie.insert("a".into(), 2);
        assert_equal(
            vec![("".into(), &1), ("a".into(), &2)].into_iter(),
            trie.iter(),
        );

        trie.insert("b".into(), 3);
        assert_equal(
            vec![("".into(), &1), ("a".into(), &2), ("b".into(), &3)].into_iter(),
            trie.iter(),
        );

        trie.insert("abc".into(), 4);
        assert_equal(
            vec![
                ("".into(), &1),
                ("a".into(), &2),
                ("abc".into(), &4),
                ("b".into(), &3),
            ]
            .into_iter(),
            trie.iter(),
        );

        trie.insert("ab".into(), 5);
        assert_equal(
            vec![
                ("".into(), &1),
                ("a".into(), &2),
                ("ab".into(), &5),
                ("abc".into(), &4),
                ("b".into(), &3),
            ]
            .into_iter(),
            trie.iter(),
        );
    }
}
