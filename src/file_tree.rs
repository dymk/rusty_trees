// use std::{
//     ffi::{OsStr, OsString},
//     io,
//     path::{Path, PathBuf},
// };

// use itertools::Itertools;

// use super::{InteriorNode, LeafNode, NodePath, Tree};

// impl NodePath for PathBuf {
//     type Component = OsString;

//     fn from_components(components: &Vec<&Self::Component>) -> Self {
//         components.iter().collect::<PathBuf>()
//     }

//     fn to_components(&self) -> Box<&dyn Iterator<Item = Self::Component>> {
//         todo!()
//     }
// }

// impl<'a, V> Tree<'a, PathBuf, V> {
//     pub fn new(root: &str) -> Result<Self, io::Error> {
//         let root = PathBuf::from(root);
//         Self::for_file_or_dir(&root, root.as_os_str()) /*.map(|mut node| {
//                                                            match &mut node {
//                                                                Tree::Interior(interior) => interior.component = root,
//                                                                Tree::Leaf(leaf) => leaf.component = root,
//                                                            }
//                                                            node
//                                                        })*/
//     }

//     fn for_file_or_dir(path: &Path, name: &OsStr) -> Result<Self, io::Error> {
//         if path.is_dir() {
//             Self::for_dir(path, name)
//         } else {
//             Self::for_file(path, name)
//         }
//     }

//     fn for_dir(path: &Path, name: &OsStr) -> Result<Self, io::Error> {
//         let paths = std::fs::read_dir(&path)?;

//         let children = paths
//             .sorted_by_key(|result| match result {
//                 Ok(path) => path.path(),
//                 Err(_) => PathBuf::new(),
//             })
//             .map(|path| {
//                 let path = path?;
//                 let full_path = path.path();
//                 let file_name = path.file_name();
//                 Self::for_file_or_dir(&full_path, &file_name)
//             })
//             .collect::<Result<Vec<_>, io::Error>>()?;

//         Ok(Tree::Interior(InteriorNode {
//             value: None,
//             component: name.to_owned(),
//             children,
//             phantom: Default::default(),
//         }))
//     }

//     fn for_file(path: &Path, name: &OsStr) -> Result<Self, io::Error> {
//         Ok(Tree::Leaf(LeafNode {
//             value: None,
//             component: name.to_owned(),
//         }))
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::path::PathBuf;

//     use itertools::assert_equal;

//     use super::Tree;

//     #[test]
//     fn test_name_and_path() {
//         let tree = Tree::<PathBuf, ()>::new("fixtures/test_tree").unwrap();
//     }

//     #[test]
//     fn test_iterator() {
//         let tree = Tree::<PathBuf, ()>::new("fixtures/test_tree").unwrap();
//         assert_equal(
//             vec![
//                 "fixtures/test_tree",
//                 "fixtures/test_tree/a_dir",
//                 "fixtures/test_tree/a_dir/bar.txt",
//                 "fixtures/test_tree/a_dir/baz.txt",
//                 "fixtures/test_tree/foo.txt",
//             ]
//             .iter()
//             .map(|s| (PathBuf::from(s), None)),
//             tree.iter(),
//         )
//     }
// }
