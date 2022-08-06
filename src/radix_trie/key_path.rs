use std::borrow::Borrow;

/// Trait that the key for a RadixTrie must implement
/// `Ref` is the "reference" type of the key, for String this would be &str,
/// for e.g. PathBuf this would be Path
pub trait Path: Borrow<Self::Ref> {
    /// Reference type to the path. E.g. for String, Ref would be &str
    type Ref: PathRef<Self> + ?Sized;

    /// Concatenate this path with another
    fn concat(self, other: Self) -> Self;
}

/// Trait that the key's Ref type must implement.
/// All operations are performed on references to the keys within the trie,
/// and not the owned values themselves.
pub trait PathRef<Path: ?Sized>: ToOwned<Owned = Path> {
    /// Is the path empty?
    fn is_empty(&self) -> bool;

    /// For paths `a` and `b`, return the common prefix between the two, and the
    /// remaining parts of the keys that remain
    fn prefix<'a>(a: &'a Self, b: &'a Self) -> (&'a Self, &'a Self, &'a Self);

    /// Concatenate an iterator of PathRef into an owned Path
    fn concat(iter: &mut dyn Iterator<Item = &Self>) -> Path;
}
