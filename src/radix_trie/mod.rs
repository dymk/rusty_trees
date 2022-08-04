use std::{borrow::Borrow, mem};

use self::key_path::{Path, PathRefType};

mod debug_impl;
mod key_path;
mod key_path_string_impl;

/**
 * A RadixTrie consists of a list of nodes, which have key paths that
 * share no common prefixes amongst themselves.
 */
pub struct RadixTrie<P, V> {
    value: Option<V>,
    nodes: Vec<Node<P, V>>,
}

struct Node<P, V> {
    path: P,
    trie: RadixTrie<P, V>,
}

impl<P, V> RadixTrie<P, V> {
    pub fn new() -> RadixTrie<P, V> {
        Self::with_value_and_capacity(None, 0)
    }
    pub fn with_value(value: V) -> RadixTrie<P, V> {
        Self::with_value_and_capacity(Some(value), 0)
    }
    pub fn with_value_and_capacity(value: Option<V>, n: usize) -> RadixTrie<P, V> {
        RadixTrie {
            value,
            nodes: Vec::with_capacity(n),
        }
    }
}

impl<P, V> Default for RadixTrie<P, V> {
    fn default() -> Self {
        Self::new()
    }
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
        let path: &P::Ref = path.borrow();
        self.get_impl(path)
    }

    fn get_impl(&self, path: &P::Ref) -> Option<&V> {
        if path.is_empty() {
            return self.value.as_ref();
        }

        for node in &self.nodes {
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            let (prefix_empty, node_rest_empty, _path_rest_empty) = (
                prefix.is_empty(),
                node_rest.is_empty(),
                path_rest.is_empty(),
            );

            // no prefix match, skip this node
            if prefix_empty {
                continue;
            }

            if node_rest_empty {
                // consumed the whole child path, delegate getting to the child
                return node.trie.get_impl(path_rest);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, path: P, value: V) -> Option<V> {
        let ret = self.insert_impl(path.borrow(), value);
        self.check_nodes_prefix_invariant();
        ret
    }

    fn insert_impl<'a>(&'a mut self, path: &'a P::Ref, value: V) -> Option<V> {
        // path is empty - insert at this node
        if path.is_empty() {
            return self.value.replace(value);
        }

        for node in &mut self.nodes {
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            let (prefix_empty, node_rest_empty, path_rest_empty) = (
                prefix.is_empty(),
                node_rest.is_empty(),
                path_rest.is_empty(),
            );

            if prefix_empty {
                // no common prefix, skip this node
                continue;
            }

            match (node_rest_empty, path_rest_empty) {
                // found exact target node
                (true, true) => {
                    return node.trie.insert_impl(path_rest, value);
                }

                // stopped in an interior that does not yet exist, and want to go down another path
                (false, false) => {
                    // create the new fork in the road

                    let interior_trie = RadixTrie::with_value_and_capacity(None, 2);

                    let left_fork = Node {
                        path: node_rest.to_owned(),
                        trie: mem::replace(&mut node.trie, interior_trie),
                    };

                    let right_fork = Node {
                        path: path_rest.to_owned(),
                        trie: RadixTrie::with_value(value),
                    };

                    node.path = prefix.to_owned();
                    node.trie.nodes.push(left_fork);
                    node.trie.nodes.push(right_fork);

                    return None;
                }

                // stopped at an interior node, but not creating a fork
                (false, true) => {
                    // create the new interior node
                    let new_child_node = Node {
                        path: node_rest.to_owned(),
                        trie: mem::replace(
                            &mut node.trie,
                            RadixTrie::with_value_and_capacity(Some(value), 1),
                        ),
                    };

                    node.path = prefix.to_owned();
                    node.trie.nodes.push(new_child_node);

                    return None;
                }
                // no more of this child, but there is more path -
                // recurse
                (true, false) => {
                    return node.trie.insert_impl(path_rest, value);
                }
            }
        }

        // no relevant existing child node found, insert as a new subnode
        self.nodes.push(Node {
            path: path.to_owned(),
            trie: RadixTrie::with_value(value),
        });

        None
    }

    /**
     * Sanity check - ensure that the prefix invariant holds. No two nodes of
     * a single trie should share a common prefix - if they did, that indicates
     * we did not create an interior node of that common prefix.
     */
    #[cfg(not(debug_assertions))]
    fn check_nodes_prefix_invariant(&self) {}

    #[cfg(debug_assertions)]
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
            value: None,
            nodes: vec![Node {
                path: "do".into(),
                trie: RadixTrie {
                    value: None,
                    nodes: vec![
                        Node {
                            path: "g".into(),
                            trie: RadixTrie::with_value(1),
                        },
                        Node {
                            path: "ts".into(),
                            trie: RadixTrie::with_value(2),
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
        assert_eq!(Some(&1), trie.get("dog"));
        assert_eq!(Some(&1), trie.get(&"dog".to_owned()));
        assert_eq!(Some(&2), trie.get("dots"));
        assert_eq!(None, trie.get("dolt"));
    }

    #[test]
    fn test_insert() {
        let mut trie = get_test_trie();
        // split a node
        assert_eq!(None, trie.insert("d".into(), 9));
        assert_eq!(Some(&9), trie.get("d"));
        assert_eq!(None, trie.get("do"));
        assert_eq!(Some(&1), trie.get("dog"));

        // split a node, check right value is returned
        assert_eq!(Some(1), trie.insert("dog".into(), 10));
        assert_eq!(Some(&10), trie.get("dog"));

        // create a new forking node
        assert_eq!(None, trie.insert("dotty".into(), 11));
        assert_eq!(Some(&11), trie.get("dotty"));
    }

    #[test]
    fn test_fuzzer_1() {
        let mut trie: RadixTrie<String, ()> = RadixTrie::new();
        assert_eq!(None, trie.insert("".to_owned(), ()));
        assert_eq!(Some(()), trie.insert("".to_owned(), ()));
    }

    #[test]
    fn test_fuzzer_2() {
        let mut trie: RadixTrie<String, ()> = RadixTrie::new();
        assert_eq!(None, trie.insert("k".to_owned(), ()));
        assert_eq!(None, trie.insert("a".to_owned(), ()));
        assert_eq!(None, trie.insert("".to_owned(), ()));
        assert_eq!(Some(()), trie.insert("a".to_owned(), ()));
    }
}
