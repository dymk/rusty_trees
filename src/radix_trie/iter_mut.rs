use crate::radix_trie::{Key, KeyRef, RadixTrie};

use super::Node;

/// Mutable iterator over a [RadixTrie]
///
/// Yielded items are a tuple of (P, &mut V), where
/// P is the key type, and V is the value type
pub struct IterMut<'a, P, V>(Vec<IterStateMut<'a, P, V>>)
where
    P: Key;

pub struct IterStateMut<'a, P, V>
where
    P: Key,
{
    key: Option<&'a P::Ref>,
    value: Option<&'a mut V>,
    nodes: Option<&'a mut [Node<P, V>]>,
}

impl<'a, P, V> IterMut<'a, P, V>
where
    P: Key,
{
    pub(super) fn new(tree: &'a mut RadixTrie<P, V>) -> Self {
        IterMut(vec![Self::to_iter_state(None, tree)])
    }

    fn to_iter_state(
        key: Option<&'a P::Ref>,
        tree: &'a mut RadixTrie<P, V>,
    ) -> IterStateMut<'a, P, V> {
        IterStateMut {
            key,
            value: tree.value.as_mut(),
            nodes: Some(&mut tree.nodes[..]),
        }
    }
}

impl<'a, P, V> Iterator for IterMut<'a, P, V>
where
    P: Key,
{
    type Item = (P, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.last_mut().and_then(|node_it| node_it.next()) {
                Some(IterStateItemMut::Value(value)) => {
                    let mut key_iter = self.0.iter().filter_map(|e| e.key);
                    let key = P::Ref::concat(&mut key_iter);
                    return Some((key, value));
                }
                Some(IterStateItemMut::Trie(key, trie)) => {
                    self.0.push(Self::to_iter_state(Some(key), trie))
                }
                None => {
                    self.0.pop()?;
                }
            }
        }
    }
}

pub enum IterStateItemMut<'a, P, V>
where
    P: Key,
{
    Value(&'a mut V),
    Trie(&'a P::Ref, &'a mut RadixTrie<P, V>),
}

impl<'a, P, V> Iterator for IterStateMut<'a, P, V>
where
    P: Key,
{
    type Item = IterStateItemMut<'a, P, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(IterStateItemMut::Value(value));
        }
        let nodes = self.nodes.take()?;
        let (head, rest) = nodes.split_first_mut()?;
        self.nodes = Some(rest);
        return Some(IterStateItemMut::Trie(head.key.borrow(), &mut head.trie));
    }
}

#[cfg(test)]
mod test {
    use itertools::assert_equal;

    use crate::radix_trie::RadixTrie;

    #[test]
    fn test_works() {
        let mut trie = RadixTrie::<String, _>::new();
        trie.insert("ab".into(), 1);
        trie.insert("abc".into(), 2);
        trie.insert("abd".into(), 3);
        trie.insert("".into(), 4);

        for (_, elem) in trie.iter_mut() {
            *elem += 1;
        }

        assert_equal(
            vec![
                ("".into(), &5),
                ("ab".into(), &2),
                ("abc".into(), &3),
                ("abd".into(), &4),
            ]
            .into_iter(),
            trie.iter(),
        );
    }
}
