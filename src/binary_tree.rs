use std::collections::VecDeque;

struct Node<T> {
    elem: T,
    children: Vec<Node<T>>,
}

struct NodeIterMut<'a, T: 'a> {
    elem: Option<&'a mut T>,
    children: VecDeque<&'a mut Node<T>>,
}

enum State<'a, T: 'a> {
    Elem(&'a mut T),
    Node(&'a mut Node<T>),
}

pub struct IterMut<'a, T: 'a>(VecDeque<NodeIterMut<'a, T>>);

impl<T> Node<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut deque = VecDeque::new();
        deque.push_front(self.iter_state());
        IterMut(deque)
    }
}

impl<T> Node<T> {
    fn iter_state(&mut self) -> NodeIterMut<T> {
        NodeIterMut {
            elem: Some(&mut self.elem),
            children: self.children.iter_mut().collect(),
        }
    }
}

impl<'a, T> Iterator for NodeIterMut<'a, T> {
    type Item = State<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elem.take() {
            Some(node) => Some(State::Elem(node)),
            None => match self.children.pop_front() {
                Some(node) => Some(State::Node(node)),
                None => None,
            },
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.front_mut().and_then(|node_it| node_it.next()) {
                Some(State::Elem(elem)) => return Some(elem),
                Some(State::Node(node)) => self.0.push_front(node.iter_state()),
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
    use itertools::Itertools;

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

        assert_eq!(vec![2, 3, 4], tree.iter_mut().map(|v| *v).collect_vec());
    }
}
