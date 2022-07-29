use crate::{IntoComponents, NodePath, Trie};

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
}
