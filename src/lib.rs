use std::marker::PhantomData;

// use self::{tree_iterator::TreeIterator, tree_iterator_mut::TreeIteratorMut};
use self::{tree_iterator::TreeIterator};

pub mod file_tree;
mod tree_iterator;
mod tree_iterator_mut;
mod binary_tree;

pub trait NodePath {
    type Component;
    fn from_components(components: &Vec<&Self::Component>) -> Self;
    fn to_components(&self) -> Box<&dyn Iterator<Item = Self::Component>>;
}

pub struct InteriorNode<'a, P, T>
where
    P: NodePath,
{
    component: P::Component,
    value: Option<T>,
    children: Vec<Tree<'a, P, T>>,
    phantom: PhantomData<&'a T>,
}
pub struct LeafNode<P, T>
where
    P: NodePath,
{
    component: P::Component,
    value: Option<T>,
}

pub enum Tree<'a, P, T>
where
    P: NodePath,
{
    Interior(InteriorNode<'a, P, T>),
    Leaf(LeafNode<P, T>),
}

impl<'a, P, V> Tree<'a, P, V>
where
    P: NodePath,
{
    fn iter(&self) -> impl Iterator<Item = (P, Option<&V>)> + '_ {
        TreeIterator::new(self)
    }
    // fn iter_mut(&'a mut self) -> impl Iterator<Item = (P, &'a mut Option<V>)> + '_ {
    //     TreeIteratorMut::new(self)
    // }
}


