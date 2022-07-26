use crate::{IntoComponents, NodePath};

impl NodePath for String {
    type Component = char;

    fn from_components(components: &mut dyn Iterator<Item = &Self::Component>) -> Self {
        components.collect()
    }

    fn to_components(&self) -> Box<dyn Iterator<Item = Self::Component> + '_> {
        Box::new(self.chars())
    }
}

impl IntoComponents<String> for String {
    fn into_components(&self) -> Vec<char> {
        self.to_components().collect()
    }
}
impl IntoComponents<String> for &str {
    fn into_components(&self) -> Vec<char> {
        self.chars().collect()
    }
}
impl IntoComponents<String> for char {
    fn into_components(&self) -> Vec<char> {
        vec![*self]
    }
}
