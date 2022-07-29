use crate::{NodePath, Trie};

pub struct Iter<'a, P, T>(Vec<NodeIter<'a, P, T>>)
where
    P: NodePath;

impl<'a, P, T> Iter<'a, P, T>
where
    P: NodePath,
{
    pub(crate) fn new(tree: &'a Trie<P, T>) -> Self {
        let mut deque = Vec::new();
        deque.push(Self::node_iter(tree));
        Iter(deque)
    }

    fn node_iter(tree: &'a Trie<P, T>) -> NodeIter<'a, P, T> {
        NodeIter {
            component: &tree.component,
            value: tree.value.as_ref(),
            children: Some(&tree.children[..]),
        }
    }
}

impl<'a, P, T> Iterator for Iter<'a, P, T>
where
    P: NodePath,
{
    type Item = (P, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.last_mut().and_then(|node_it| node_it.next()) {
                Some(State::Elem(elem)) => {
                    let mut path_iter = self.0.iter().map(|e| e.component);
                    let path = P::from_components(&mut path_iter);
                    return Some((path, elem));
                }
                Some(State::Node(node)) => self.0.push(Self::node_iter(node)),
                None => {
                    if let None = self.0.pop() {
                        return None;
                    }
                }
            }
        }
    }
}

struct NodeIter<'a, P, T>
where
    P: NodePath,
{
    component: &'a P::Component,
    value: Option<&'a T>,
    children: Option<&'a [Trie<P, T>]>,
}

enum State<'a, P, T: 'a>
where
    P: NodePath,
{
    Elem(&'a T),
    Node(&'a Trie<P, T>),
}

impl<'a, P, T> Iterator for NodeIter<'a, P, T>
where
    P: NodePath,
{
    type Item = State<'a, P, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value.take() {
            Some(node) => Some(State::Elem(node)),
            None => {
                let children = self.children.take().unwrap();
                match children.split_first() {
                    Some((head, rest)) => {
                        self.children = Some(rest);
                        Some(State::Node(head))
                    }
                    None => None,
                }
            }
        }
    }
}
