use crate::radix_trie::{Path, PathRef, RadixTrie};

use super::Node;

/// Iterator over a [RadixTrie]
///
/// Yielded items are a tuple of (P, &V), where
/// P is the path type, and V is the value type
pub struct Iter<'a, P, V>(Vec<IterState<'a, P, V>>)
where
    P: Path;

pub struct IterState<'a, P, V>
where
    P: Path,
{
    path: Option<&'a P::Ref>,
    value: Option<&'a V>,
    nodes: Option<&'a [Node<P, V>]>,
}

impl<'a, P, V> Iter<'a, P, V>
where
    P: Path,
{
    pub(super) fn new(tree: &'a RadixTrie<P, V>) -> Self {
        Iter(vec![Self::to_iter_state(None, tree)])
    }

    fn to_iter_state(path: Option<&'a P::Ref>, tree: &'a RadixTrie<P, V>) -> IterState<'a, P, V> {
        IterState {
            path,
            value: tree.value.as_ref(),
            nodes: Some(&tree.nodes[..]),
        }
    }
}

impl<'a, P, V> Iterator for Iter<'a, P, V>
where
    P: Path,
{
    type Item = (P, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.last_mut().and_then(|node_it| node_it.next()) {
                Some(IterStateItem::Value(value)) => {
                    let mut path_iter = self.0.iter().filter_map(|e| e.path);
                    let path = P::Ref::concat(&mut path_iter);
                    return Some((path, value));
                }
                Some(IterStateItem::Trie(path, trie)) => {
                    self.0.push(Self::to_iter_state(Some(path), trie))
                }
                None => {
                    if let None = self.0.pop() {
                        return None;
                    }
                }
            }
        }
    }
}

pub enum IterStateItem<'a, P, V>
where
    P: Path,
{
    Value(&'a V),
    Trie(&'a P::Ref, &'a RadixTrie<P, V>),
}

impl<'a, P, V> Iterator for IterState<'a, P, V>
where
    P: Path,
{
    type Item = IterStateItem<'a, P, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(IterStateItem::Value(value));
        }
        let nodes = if let Some(nodes) = self.nodes.take() {
            nodes
        } else {
            return None;
        };
        let (head, rest) = if let Some(pair) = nodes.split_first() {
            pair
        } else {
            return None;
        };
        self.nodes = Some(rest);
        return Some(IterStateItem::Trie(head.path.borrow(), &head.trie));
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
