use std::{borrow::Borrow, mem};

use self::key_path::{Path, PathRefType};

mod debug_impl;
mod key_path;
mod key_path_string_impl;

/**
 * A RadixTrie consists of a list of nodes, which have key paths that
 * share no common prefixes amongst themselves.
 */
#[derive(Default)]
pub struct RadixTrie<P, V> {
    nodes: Vec<Node<P, V>>,
}

struct Node<P, V> {
    path: P,
    value: Option<V>,
    trie: RadixTrie<P, V>,
}

impl<P, V> RadixTrie<P, V>
where
    P: Path,
{
    #[allow(dead_code)]
    pub fn get<Q>(&self, path: &Q) -> Option<&V>
    where
        Q: Borrow<P::Ref> + ?Sized,
    {
        for node in &self.nodes {
            let path: &P::Ref = path.borrow();
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            if prefix.is_empty() {
                continue;
            }

            return match (node_rest.is_empty(), path_rest.is_empty()) {
                // consumed whole child path, but there's more path to go - recurse
                (true, false) => node.trie.get(path_rest),
                // consumed whole child path, and no more to go - this is the target node
                (true, true) => node.value.as_ref(),
                // stopped in the middle of the child, implicitly an empty node
                (false, true) => None,
                // stopped in the middle of the child, wanted to go down a nonexistent node path
                (false, false) => None,
            };
        }

        None
    }

    #[allow(dead_code)]
    pub fn put(&mut self, path: P, value: V) -> Option<V> {
        let ret = self.put_impl(path.borrow(), value);
        self.check_nodes_prefix_invariant();
        ret
    }

    fn put_impl<'a>(&'a mut self, path: &'a P::Ref, value: V) -> Option<V> {
        for node in &mut self.nodes {
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            if prefix.is_empty() {
                continue;
            }

            match (node_rest.is_empty(), path_rest.is_empty()) {
                // found exact target node
                (true, true) => return node.value.replace(value),

                // stopped in an interior that does not yet exist, and want to go down another path
                (false, false) => {
                    // create the new fork in the road
                    let mut new_parent_node = Node {
                        path: prefix.to_owned(),
                        trie: RadixTrie {
                            nodes: vec![
                                Node {
                                    path: node_rest.to_owned(),
                                    value: None,
                                    trie: RadixTrie { nodes: vec![] },
                                },
                                Node {
                                    path: path_rest.to_owned(),
                                    value: Some(value),
                                    trie: RadixTrie { nodes: vec![] },
                                },
                            ],
                        },
                        value: None,
                    };

                    Self::swap_into_first_child(node, &mut new_parent_node);
                    return None;
                }

                // stopped at an interior node, but not creating a fork
                (false, true) => {
                    // create the new interior node
                    let mut new_parent_node = Node {
                        path: prefix.to_owned(),
                        trie: RadixTrie {
                            nodes: vec![Node {
                                path: node_rest.to_owned(),
                                value: None,
                                trie: RadixTrie { nodes: vec![] },
                            }],
                        },
                        value: Some(value),
                    };

                    Self::swap_into_first_child(node, &mut new_parent_node);
                    return None;
                }
                // no more of this child, but there is more path -
                // recurse
                (true, false) => return node.trie.put_impl(path_rest, value),
            }
        }

        // no relevant existing child node found, insert as a new subnode
        self.nodes.push(Node {
            path: path.to_owned(),
            value: Some(value),
            trie: RadixTrie { nodes: vec![] },
        });

        None
    }

    // Splices `into_this` into the tree, by making `put_this` its
    // first child and putting itself into `put_this`'s place.
    fn swap_into_first_child(put_this: &mut Node<P, V>, into_this: &mut Node<P, V>) {
        let first_child = &mut into_this.trie.nodes.get_mut(0).unwrap();
        mem::swap(&mut first_child.value, &mut put_this.value);
        mem::swap(&mut first_child.trie, &mut put_this.trie);
        mem::swap(put_this, into_this);
    }

    /**
     * Sanity check - ensure that the prefix invariant holds. No two nodes of
     * a single trie should share a common prefix - if they did, that indicates
     * we did not create an interior node of that common prefix.
     */
    fn check_nodes_prefix_invariant(&self) {
        for (idx1, n1) in self.nodes.iter().enumerate() {
            for (idx2, n2) in self.nodes.iter().enumerate() {
                if idx1 == idx2 {
                    continue;
                }

                let (prefix, _, _) = P::Ref::prefix(n1.path.borrow(), n2.path.borrow());
                if !P::Ref::is_empty(prefix) {
                    panic!("Invariant not held, prefix shared in nodes");
                }
            }
        }

        for node in &self.nodes {
            node.trie.check_nodes_prefix_invariant();
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Node, RadixTrie};

    fn get_test_trie() -> RadixTrie<String, i32> {
        let trie = RadixTrie {
            nodes: vec![Node {
                path: "do".into(),
                value: None,
                trie: RadixTrie {
                    nodes: vec![
                        Node {
                            path: "g".into(),
                            value: Some(2),
                            trie: RadixTrie::default(),
                        },
                        Node {
                            path: "ts".into(),
                            value: Some(3),
                            trie: RadixTrie::default(),
                        },
                    ],
                },
            }],
        };
        trie.check_nodes_prefix_invariant();
        trie
    }

    #[test]
    fn test_get() {
        let trie = get_test_trie();

        assert_eq!(None, trie.get(""));
        assert_eq!(None, trie.get("a"));
        assert_eq!(None, trie.get("as"));
        assert_eq!(None, trie.get("do"));
        assert_eq!(Some(&2), trie.get("dog"));
        assert_eq!(Some(&2), trie.get(&"dog".to_owned()));
        assert_eq!(Some(&3), trie.get("dots"));
        assert_eq!(None, trie.get("dolt"));
    }

    #[test]
    fn test_insert() {
        let mut trie = get_test_trie();
        // split a node
        assert_eq!(None, trie.put("d".into(), 9));
        assert_eq!(Some(&9), trie.get("d"));
        assert_eq!(None, trie.get("do"));
        assert_eq!(Some(&2), trie.get("dog"));

        // split a node, check right value is returned
        assert_eq!(Some(2), trie.put("dog".into(), 10));
        assert_eq!(Some(&10), trie.get("dog"));

        // create a new forking node
        assert_eq!(None, trie.put("dotty".into(), 11));
        assert_eq!(Some(&11), trie.get("dotty"));
    }
}
