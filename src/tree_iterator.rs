use super::{NodePath, Tree};

pub struct TreeIterator<'a, P, V>
where
    P: NodePath,
{
    file_tree_node_stack: Vec<(usize, &'a Tree<'a, P, V>)>,
    components_stack: Vec<&'a P::Component>,
}

impl<'a, P, V> TreeIterator<'a, P, V>
where
    P: NodePath,
{
    pub(super) fn new(file_tree: &'a Tree<P, V>) -> TreeIterator<'a, P, V> {
        TreeIterator {
            file_tree_node_stack: vec![(0, file_tree)],
            components_stack: vec![],
        }
    }
}

enum IteratorAction<'a, P, V>
where
    P: NodePath,
{
    Return(Option<(P, Option<&'a V>)>),
    PushChild(&'a Tree<'a, P, V>),
}

impl<'a, P, V> Iterator for TreeIterator<'a, P, V>
where
    P: NodePath,
{
    type Item = (P, Option<&'a V>);

    fn next(&mut self) -> Option<Self::Item> {
        let action: IteratorAction<P, V> = match self.file_tree_node_stack.last_mut() {
            Some((idx, node)) => match node {
                Tree::Interior(dir) => {
                    if *idx == 0 {
                        // zero idx - visit self
                        *idx += 1;
                        self.components_stack.push(&dir.component);
                        let path = P::from_components(&self.components_stack);
                        IteratorAction::Return(Some((path, dir.value.as_ref())))
                    } else if *idx == dir.children.len() + 1 {
                        // very last idx - pop to go to next node
                        self.file_tree_node_stack.pop();
                        self.components_stack.pop();
                        IteratorAction::Return(self.next())
                    } else {
                        // middle idx - push the `idx-1`th child and visit it
                        let child = dir.children.iter().nth((*idx) - 1).unwrap();
                        *idx += 1;
                        IteratorAction::PushChild(child)
                    }
                }
                Tree::Leaf(file) => {
                    self.file_tree_node_stack.pop();
                    self.components_stack.push(&file.component);
                    let path = P::from_components(&self.components_stack);
                    self.components_stack.pop();
                    IteratorAction::Return(Some((path, file.value.as_ref())))
                }
            },
            None => IteratorAction::Return(None),
        };

        match action {
            IteratorAction::Return(ret) => ret,
            IteratorAction::PushChild(child) => {
                self.file_tree_node_stack.push((0, child));
                self.next()
            }
        }
    }
}
