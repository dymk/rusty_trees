use crate::radix_trie::{Path, PathRef, RadixTrie};

use super::Node;

pub struct IterMut<'a, P, V>(Vec<IterStateMut<'a, P, V>>)
where
    P: Path;

pub struct IterStateMut<'a, P, V>
where
    P: Path,
{
    path: Option<&'a P::Ref>,
    value: Option<&'a mut V>,
    nodes: Option<&'a mut [Node<P, V>]>,
}

impl<'a, P, V> IterMut<'a, P, V>
where
    P: Path,
{
    pub(super) fn new(tree: &'a mut RadixTrie<P, V>) -> Self {
        IterMut(vec![Self::to_iter_state(None, tree)])
    }

    fn to_iter_state(
        path: Option<&'a P::Ref>,
        tree: &'a mut RadixTrie<P, V>,
    ) -> IterStateMut<'a, P, V> {
        IterStateMut {
            path,
            value: tree.value.as_mut(),
            nodes: Some(&mut tree.nodes[..]),
        }
    }
}

impl<'a, P, V> Iterator for IterMut<'a, P, V>
where
    P: Path,
{
    type Item = (P, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.last_mut().and_then(|node_it| node_it.next()) {
                Some(IterStateItemMut::Value(value)) => {
                    let mut path_iter = self.0.iter().filter_map(|e| e.path);
                    let path = P::Ref::concat(&mut path_iter);
                    return Some((path, value));
                }
                Some(IterStateItemMut::Trie(path, trie)) => {
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

pub enum IterStateItemMut<'a, P, V>
where
    P: Path,
{
    Value(&'a mut V),
    Trie(&'a P::Ref, &'a mut RadixTrie<P, V>),
}

impl<'a, P, V> Iterator for IterStateMut<'a, P, V>
where
    P: Path,
{
    type Item = IterStateItemMut<'a, P, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(IterStateItemMut::Value(value));
        }
        let nodes = if let Some(nodes) = self.nodes.take() {
            nodes
        } else {
            return None;
        };
        let (head, rest) = if let Some(pair) = nodes.split_first_mut() {
            pair
        } else {
            return None;
        };
        self.nodes = Some(rest);
        return Some(IterStateItemMut::Trie(head.path.borrow(), &mut head.trie));
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
