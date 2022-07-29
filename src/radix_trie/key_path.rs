use std::borrow::Borrow;

/**
 * Trait that the key for a RadixTrie must implement
 * `Ref` is the "reference" type of the key, for String this would be &str,
 * for e.g. PathBuf this would be Path
 */
pub trait Path: Borrow<Self::Ref> {
    type Ref: PathRefType<Self> + ?Sized;
}

/**
 * Trait that the key's Ref type must implement.
 * All operations are performed on references to the keys within the trie,
 * and not the owned values themselves.
 */
pub trait PathRefType<Owned: ?Sized> {
    /**
     * Convert the reference to its Owned type (likely by clone / copy)
     */
    fn to_owned(&self) -> Owned;
    /**
     * Is the key empty?
     */
    fn is_empty(&self) -> bool;
    /**
     * For keys `a` and `b`, return the common prefix between the two, and the
     * remaining parts of the keys that remain
     */
    fn prefix<'a>(a: &'a Self, b: &'a Self) -> (&'a Self, &'a Self, &'a Self);
}
