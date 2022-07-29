macro_rules! trie {
    ($path:expr) => {
        $crate::Trie::from_component($path)
    };
    ($path:expr, $val:expr) => {
        $crate::Trie::from_component_val($path, $val)
    };
    ($path:expr, $val:expr; $($tries:expr),* $(,)?) => {
        $crate::Trie::from_component_val_children($path, $val, vec![$($tries),+])
    };
    ($path:expr; $($tries:expr),* $(,)?) => {
        $crate::Trie::from_component_children($path, vec![$($tries),+])
    }
}
