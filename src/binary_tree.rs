use std::collections::VecDeque;

pub struct Node<T> {
    elem: T,
    children: Vec<Node<T>>,
}

struct NodeIterMut<'a, T> {
    elem: Option<&'a mut T>,
    children: Option<&'a mut [Node<T>]>,
}

struct NodeIter<'a, T> {
    elem: Option<&'a T>,
    children: Option<&'a [Node<T>]>,
}

pub struct IterMut<'a, T: 'a>(VecDeque<NodeIterMut<'a, T>>);
pub struct Iter<'a, T: 'a>(VecDeque<NodeIter<'a, T>>);

impl<T> Node<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut deque = VecDeque::new();
        deque.push_front(self.node_iter_mut());
        IterMut(deque)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut deque = VecDeque::new();
        deque.push_front(self.node_iter());
        Iter(deque)
    }

    fn node_iter_mut(&mut self) -> NodeIterMut<T> {
        NodeIterMut {
            elem: Some(&mut self.elem),
            children: Some(&mut self.children[..]),
        }
    }

    fn node_iter(&self) -> NodeIter<T> {
        NodeIter {
            elem: Some(&self.elem),
            children: Some(&self.children[..]),
        }
    }
}

enum StateMut<'a, T: 'a> {
    Elem(&'a mut T),
    Node(&'a mut Node<T>),
}

enum State<'a, T: 'a> {
    Elem(&'a T),
    Node(&'a Node<T>),
}

impl<'a, T> Iterator for NodeIterMut<'a, T> {
    type Item = StateMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elem.take() {
            Some(node) => Some(StateMut::Elem(node)),
            None => {
                let children = self.children.take().unwrap();
                match children.split_first_mut() {
                    Some((head, rest)) => {
                        self.children = Some(rest);
                        Some(StateMut::Node(head))
                    }
                    None => None,
                }
            }
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = State<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elem.take() {
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.front_mut().and_then(|node_it| node_it.next()) {
                Some(StateMut::Elem(elem)) => return Some(elem),
                Some(StateMut::Node(node)) => self.0.push_front(node.node_iter_mut()),
                None => {
                    if let None = self.0.pop_front() {
                        return None;
                    }
                }
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.front_mut().and_then(|node_it| node_it.next()) {
                Some(State::Elem(elem)) => return Some(elem),
                Some(State::Node(node)) => self.0.push_front(node.node_iter()),
                None => {
                    if let None = self.0.pop_front() {
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::{assert_equal, Itertools};

    use super::Node;

    #[test]
    fn test_works() {
        let mut tree = Node {
            elem: 1,
            children: vec![
                Node {
                    elem: 2,
                    children: vec![],
                },
                Node {
                    elem: 3,
                    children: vec![],
                },
            ],
        };

        tree.iter_mut().for_each(|value| {
            *value += 1;
        });

        assert_equal(vec![2, 3, 4].iter(), tree.iter());
    }
}
