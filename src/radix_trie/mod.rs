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

    fn with_value(value: V) -> RadixTrie<P, V> {
        Self::with_value_and_capacity(Some(value), 0)
    }

    fn with_value_and_capacity(value: Option<V>, n: usize) -> RadixTrie<P, V> {
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
    pub fn get<Q>(&self, path: &Q) -> Option<&V>
    where
        Q: Borrow<P::Ref> + ?Sized,
    {
        let path: &P::Ref = path.borrow();
        let ret = self.get_impl(path);
        self.check_invariants(true);
        ret
    }

    pub fn insert(&mut self, path: P, value: V) -> Option<V> {
        let ret = self.insert_impl(path.borrow(), value);
        self.check_invariants(true);
        ret
    }

    pub fn remove<Q>(&mut self, path: &Q) -> Option<V>
    where
        Q: Borrow<P::Ref> + ?Sized,
    {
        let path: &P::Ref = path.borrow();
        let ret = self.remove_impl(0, path);
        self.check_invariants(true);
        match ret {
            RemoveResult::Skip => None,
            // for the root node, most values of ::Done are ignored, because
            // the root node itself is not dropped even if its value is None
            // and it has no children
            RemoveResult::Done {
                idx: _,
                num_children: _,
                has_value: _,
                removed_value: value,
            } => value,
        }
    }

    fn get_impl(&self, path: &P::Ref) -> Option<&V> {
        if path.is_empty() {
            return self.value.as_ref();
        }

        for node in &self.nodes {
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            let (prefix_empty, node_rest_empty) = (prefix.is_empty(), node_rest.is_empty());

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

    fn insert_impl<'a>(&'a mut self, path: &'a P::Ref, value: V) -> Option<V> {
        // path is empty, this is the exact node being targeted, insert here
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

    fn remove_impl(&mut self, this_idx: usize, path: &P::Ref) -> RemoveResult<V> {
        // path empty => this is the exact node being removed.
        // indicate to the caller that the value of this node has been moved
        // out, along with information about num_children so the parent can
        // decide if this node should be removed
        if path.is_empty() {
            return RemoveResult::Done {
                idx: this_idx,
                num_children: self.nodes.len(),
                has_value: false,
                removed_value: self.value.take(),
            };
        }

        // by default, no relevant child node was found, so skip any action
        let mut result = RemoveResult::Skip;

        for (idx, node) in self.nodes.iter_mut().enumerate() {
            let (prefix, node_rest, path_rest) = P::Ref::prefix(node.path.borrow(), path);
            let (prefix_empty, node_rest_empty) = (prefix.is_empty(), node_rest.is_empty());

            if prefix_empty {
                // no common prefix, skip this node
                continue;
            }

            if node_rest_empty {
                // node's path was entirely consumed, so go down this node path
                result = node.trie.remove_impl(idx, path_rest);
                match result {
                    RemoveResult::Skip => continue,
                    _ => break,
                };
            }
        }

        if let RemoveResult::Done {
            idx,
            num_children,
            has_value,
            removed_value: value,
        } = result
        {
            match (has_value, num_children) {
                (false, 0) => {
                    // lone empty leaf node, remove it
                    self.nodes.swap_remove(idx);
                }
                (false, 1) => {
                    // remove the interior node and extend its lone child's
                    // path
                    let mut node = self.nodes.swap_remove(idx);
                    let mut child = node.trie.nodes.pop().unwrap();
                    child.path = node.path.concat(child.path);
                    self.nodes.push(child);
                }
                _ => {
                    // node has a value, or the node has more than one child
                    // still, either way, leave it be
                }
            };

            // once the result from self.nodes[idx] has been processed, update
            // result to reflect the state of self, propagating the value
            // to the caller
            result = RemoveResult::Done {
                idx: this_idx,
                num_children: self.nodes.len(),
                has_value: self.value.is_some(),
                removed_value: value,
            };
        }

        result
    }

    /**
     * Run in debug mode, and under the fuzzer harness
     * Runtime invariants enforced on the trie structure that should always
     * hold. Hopefully tests and the fuzzer are enough to ensure that the
     * implementation is such that they always hold.
     */
    fn check_invariants(&self, is_root: bool) {
        #[cfg(debug_assertions)]
        {
            self.check_path_prefix_invariant();
            self.check_leaf_node_some_invariant(is_root);
        }
    }

    /**
     * Invariant - ensure that the prefix invariant holds. No two nodes of
     * a single trie should share a common prefix - if they did, that indicates
     * we did not create an interior node of that common prefix.
     */
    fn check_path_prefix_invariant(&self) {
        for (idx1, n1) in self.nodes.iter().enumerate() {
            for (idx2, n2) in self.nodes.iter().enumerate() {
                if idx1 == idx2 {
                    continue;
                }

                let (prefix, _, _) = P::Ref::prefix(n1.path.borrow(), n2.path.borrow());
                if !P::Ref::is_empty(prefix) {
                    panic!("no shared prefixes invariant failed");
                }
            }
        }

        for node in &self.nodes {
            node.trie.check_path_prefix_invariant();
        }
    }

    /**
     * Invariant - leaf nodes should always have a Some(_) value
     */
    fn check_leaf_node_some_invariant(&self, is_root: bool) {
        // (not relevant to root node)
        // a node that has no child nodes must be a Some(_) value
        if !is_root && self.nodes.is_empty() && self.value.is_none() {
            panic!("leaf node Some(_) invariant failed");
        }
        for node in &self.nodes {
            node.trie.check_leaf_node_some_invariant(false);
        }
    }
}

enum RemoveResult<V> {
    Skip,
    Done {
        idx: usize,
        num_children: usize,
        has_value: bool,
        removed_value: Option<V>,
    },
}

#[cfg(test)]
mod test {
    use super::{Node, RadixTrie};

    fn get_test_trie() -> RadixTrie<String, i32> {
        RadixTrie {
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
        }
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

    #[test]
    fn test_remove() {
        let mut trie: RadixTrie<String, usize> = RadixTrie::new();
        assert_eq!(None, trie.remove(""));
        assert_eq!(None, trie.remove("a"));

        assert_eq!(None, trie.insert("".to_owned(), 1));
        assert_eq!(Some(1), trie.remove(""));
        assert_eq!(None, trie.remove(""));
    }

    #[test]
    fn test_fuzzer_3() {
        let mut trie: RadixTrie<String, usize> = RadixTrie::new();
        assert_eq!(None, trie.insert("aa".to_owned(), 1));
        assert_eq!(None, trie.insert("abaa".to_owned(), 2));
        assert_eq!(None, trie.insert("ab".to_owned(), 3));
        assert_eq!(Some(3), trie.remove("ab"));
    }

    #[test]
    fn test_fuzzer_4() {
        let mut trie: RadixTrie<String, usize> = RadixTrie::new();
        assert_eq!(None, trie.insert("abb".to_owned(), 1));
        assert_eq!(None, trie.insert("ac".to_owned(), 2));
        assert_eq!(Some(1), trie.remove("abb"));
        assert_eq!(Some(2), trie.insert("ac".to_owned(), 4));
        assert_eq!(Some(4), trie.remove("ac"));
    }

    #[test]
    fn test_fuzzer_5() {
        let mut trie: RadixTrie<String, usize> = RadixTrie::new();
        assert_eq!(None, trie.insert("a".to_owned(), 1));
        assert_eq!(None, trie.insert("abc".to_owned(), 2));
        assert_eq!(Some(2), trie.remove("abc"));
        assert_eq!(Some(1), trie.insert("a".to_owned(), 3));
    }

    #[test]
    fn test_fuzzer_6() {
        let mut trie: RadixTrie<String, usize> = RadixTrie::new();
        assert_eq!(None, trie.insert("a".to_owned(), 1));
        assert_eq!(None, trie.insert("abc".to_owned(), 2));
        assert_eq!(Some(1), trie.remove("a"));
    }
}
