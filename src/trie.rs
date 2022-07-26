use std::{fmt::Debug, process::Child, thread::current};

use crate::{iter::Iter, node_path::IntoComponents, NodePath};

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
    pub fn from_component<C>(component: C) -> Self
    where
        C: IntoComponents<P>,
    {
        Self::from_components_vec(component.into_components(), None, vec![])
    }

    pub fn from_component_val<C>(component: C, value: V) -> Self
    where
        C: IntoComponents<P>,
    {
        Self::from_components_vec(component.into_components(), Some(value), vec![])
    }
    pub fn from_component_val_children<C>(component: C, value: V, children: Vec<Trie<P, V>>) -> Self
    where
        C: IntoComponents<P>,
    {
        Self::from_components_vec(component.into_components(), Some(value), children)
    }

    pub fn from_component_children<C>(component: C, children: Vec<Trie<P, V>>) -> Self
    where
        C: IntoComponents<P>,
    {
        Self::from_components_vec(component.into_components(), None, children)
    }

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

    fn from_components_vec(
        mut components: Vec<P::Component>,
        value: Option<V>,
        children: Vec<Trie<P, V>>,
    ) -> Trie<P, V> {
        components.reverse();
        Self::from_components_vec_impl(components, value, children)
    }

    fn from_components_vec_impl(
        mut components_rev: Vec<P::Component>,
        value: Option<V>,
        children: Vec<Trie<P, V>>,
    ) -> Trie<P, V> {
        if components_rev.is_empty() {
            panic!();
        }

        let component = components_rev.pop().unwrap();
        if components_rev.is_empty() {
            Trie {
                component,
                value,
                children,
            }
        } else {
            Trie {
                component,
                value: None,
                children: vec![Self::from_components_vec_impl(
                    components_rev,
                    value,
                    children,
                )],
            }
        }
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
