use std::fmt::Debug;

use crate::{iter::Iter, NodePath};
pub mod ctors;

trait TrieTrait {
    type PathType: NodePath;
    type ValueType: Debug;
}

pub struct Trie<P, V>
where
    P: NodePath,
{
    pub(crate) component: P::Component,
    pub(crate) value: Option<V>,
    pub(crate) children: Vec<Trie<P, V>>,
}

impl<P, V> TrieTrait for Trie<P, V>
where
    P: NodePath,
    V: Debug,
{
    type PathType = P;
    type ValueType = V;
}

impl<P, V> Trie<P, V>
where
    P: NodePath,
{
    pub fn iter(&self) -> Iter<P, V> {
        Iter::new(self)
    }

    pub fn num_nodes(&self) -> usize {
        let child_sum = self.children.iter().map(Trie::num_nodes).sum::<usize>();
        child_sum + 1
    }

    pub fn num_values(&self) -> usize {
        let child_sum = self.children.iter().map(Trie::num_values).sum::<usize>();
        child_sum + if self.value.is_some() { 1 } else { 0 }
    }

    pub fn get<'a, P1>(&self, path: P1) -> Option<&V>
    where
        P1: Into<P>,
    {
        let path: P = path.into();
        let mut components = path.to_components();
        let mut current_node = Some(self);

        if let Some(c) = components.next() {
            if c != self.component {
                return None;
            }
        }

        for component in components {
            match current_node {
                Some(node) => {
                    current_node = node
                        .children
                        .iter()
                        .find(|child| child.component == component)
                }
                None => return None,
            }
        }
        current_node.and_then(|node| node.value.as_ref())
    }

    // fn iter_mut(&'a mut self) -> impl Iterator<Item = (P, &'a mut Option<V>)> + '_ {
    //     TreeIteratorMut::new(self)
    // }
}

impl<P, V> Debug for Trie<P, V>
where
    P: NodePath,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Trie({} nodes, {} values):\n",
            self.num_nodes(),
            self.num_values()
        ))?;
        Self::fmt_impl(0, self, f)
    }
}
impl<P, V> Trie<P, V>
where
    P: NodePath,
    V: Debug,
{
    fn fmt_impl(ident: usize, trie: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..ident {
            f.write_str(" ")?;
        }

        f.write_str(&format!("`{:?}`", &trie.component,))?;
        if let Some(value) = &trie.value {
            f.write_str(&format!(" = `{:?}`", value))?;
        }
        f.write_str("\n")?;
        for child in trie.children.iter() {
            Self::fmt_impl(ident + 2, child, f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use itertools::assert_equal;

    use crate::{NodePath, Trie};

    #[test]
    fn test_works() {
        let trie = trie![
            "do", 1;
            trie!['g', 2],
            trie!["ts", 3],
        ];

        // println!("trie:\n{:?}", trie);
        assert_trie_iter(vec![("do", 1), ("dog", 2), ("dots", 3)], &trie);
        assert_eq!(Some(&3), trie.get("dots"));
    }

    fn assert_trie_iter<P1, P2, V>(expect: Vec<(P1, V)>, trie: &Trie<P2, V>)
    where
        P1: Into<P2> + Copy,
        P2: NodePath + Eq + Debug,
        V: Eq + Debug,
    {
        assert_equal(expect.iter().map(|(c, i)| ((*c).into(), i)), trie.iter());
    }
}
