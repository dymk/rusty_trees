use itertools::Itertools;

use super::key_path::{Path, PathRef};

/* Common key type - a String / &str */
impl Path for String {
    type Ref = str;

    fn concat(self, other: Self) -> Self {
        self + &other
    }

    fn empty() -> Self {
        "".into()
    }
}

impl PathRef<String> for str {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn prefix<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str, &'a str) {
        let prefix_len = a
            .chars()
            .zip(b.chars())
            .take_while(|(ac, bc)| ac == bc)
            .count();
        (&a[..prefix_len], &a[prefix_len..], &b[prefix_len..])
    }

    fn concat(iter: &mut dyn Iterator<Item = &str>) -> String {
        iter.join("")
    }
}

#[cfg(test)]
mod test {
    use crate::radix_trie::key_path::PathRef;

    #[test]
    fn test_works() {
        assert_eq!(("", "", ""), PathRef::prefix("", ""));
        assert_eq!(("", "a", "b"), PathRef::prefix("a", "b"));
        assert_eq!(("a", "", "b"), PathRef::prefix("a", "ab"));
        assert_eq!(("ab", "", ""), PathRef::prefix("ab", "ab"));
        assert_eq!(("foo", "123", "456"), PathRef::prefix("foo123", "foo456"));
    }
}
