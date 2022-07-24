// use super::{NodePath, Tree};

// pub struct TreeIteratorMut<'a, P, V>
// where
//     P: NodePath,
// {
//     file_tree_node_stack: Vec<(usize, &'a mut Tree<'a, P, V>)>,
//     components_stack: Vec<&'a P::Component>,
// }
// impl<'a, P, V> TreeIteratorMut<'a, P, V>
// where
//     P: NodePath,
// {
//     pub(super) fn new(file_tree: &'a mut Tree<'a, P, V>) -> TreeIteratorMut<'a, P, V> {
//         TreeIteratorMut {
//             file_tree_node_stack: vec![(0, file_tree)],
//             components_stack: vec![],
//         }
//     }
// }

// enum IteratorAction<'a, P, V>
// where
//     P: NodePath,
// {
//     Return(Option<(P, &'a mut Option<V>)>),
//     PushChild(&'a mut Tree<'a, P, V>),
// }

// impl<'a, P, V> Iterator for TreeIteratorMut<'a, P, V>
// where
//     P: NodePath,
// {
//     type Item = (P, &'a mut Option<V>);

//     fn next(&mut self) -> Option<Self::Item> {
//         let last_mut = self.file_tree_node_stack.last_mut();
//         let action: IteratorAction<P, V> = match last_mut {
//             Some((idx, node)) => match node {
//                 Tree::Interior(dir) => {
//                     if *idx == 0 {
//                         // zero idx - visit self
//                         *idx += 1;
//                         self.components_stack.push(&dir.component);
//                         let path = P::from_components(&self.components_stack);
//                         IteratorAction::Return(Some((path, &mut dir.value)))
//                     } else if *idx == dir.children.len() + 1 {
//                         // very last idx - pop to go to next node
//                         self.file_tree_node_stack.pop();
//                         self.components_stack.pop();
//                         IteratorAction::Return(self.next())
//                     } else {
//                         // middle idx - push the `idx-1`th child and visit it
//                         let child = dir.children.iter_mut().nth((*idx) - 1).unwrap();
//                         *idx += 1;
//                         IteratorAction::PushChild(child)
//                     }
//                 }
//                 Tree::Leaf(file) => {
//                     self.file_tree_node_stack.pop();
//                     self.components_stack.push(&file.component);
//                     let path = P::from_components(&self.components_stack);
//                     self.components_stack.pop();
//                     IteratorAction::Return(Some((path, &mut file.value)))
//                 }
//             },
//             None => IteratorAction::Return(None),
//         };

//         match action {
//             IteratorAction::Return(ret) => ret,
//             IteratorAction::PushChild(child) => {
//                 self.file_tree_node_stack.push((0, child));
//                 self.next()
//             }
//         }
//     }
// }
