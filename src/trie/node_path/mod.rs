pub mod string_node_path;

pub trait NodePath {
    type Component: core::fmt::Debug + PartialEq;
    fn from_components(components: &mut dyn Iterator<Item = &Self::Component>) -> Self;
    fn to_components(&self) -> Box<dyn Iterator<Item = Self::Component> + '_>;
}

pub trait IntoComponents<P>
where
    P: NodePath,
{
    fn into_components(&self) -> Vec<P::Component>;
}
