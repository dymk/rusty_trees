use super::key_path::{Path, PathRefType};

/* Common key type - a String / &str */
impl Path for String {
    type Ref = str;
}

impl PathRefType<String> for str {
    fn to_owned(&self) -> String {
        ToOwned::to_owned(self)
    }

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
}

#[cfg(test)]
mod test {
    use crate::radix_trie::key_path::PathRefType;

    #[test]
    fn test_works() {
        assert_eq!(("", "", ""), PathRefType::prefix("", ""));
        assert_eq!(("", "a", "b"), PathRefType::prefix("a", "b"));
        assert_eq!(("a", "", "b"), PathRefType::prefix("a", "ab"));
        assert_eq!(("ab", "", ""), PathRefType::prefix("ab", "ab"));
        assert_eq!(
            ("foo", "123", "456"),
            PathRefType::prefix("foo123", "foo456")
        );
    }
}
