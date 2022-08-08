use itertools::Itertools;

use super::key::{Key, KeyRef};

/* Common key type - a String / &str */
impl Key for String {
    type Ref = str;

    fn concat(self, other: Self) -> Self {
        self + &other
    }
}

impl KeyRef<String> for str {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn prefix<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str, &'a str) {
        let mut prefix_len = a.bytes().zip(b.bytes()).take_while(|(a, b)| a == b).count();

        while !a.is_char_boundary(prefix_len) {
            prefix_len -= 1;
        }

        (&a[..prefix_len], &a[prefix_len..], &b[prefix_len..])
    }

    fn concat(iter: &mut dyn Iterator<Item = &str>) -> String {
        iter.join("")
    }
}

#[cfg(test)]
mod test {
    use crate::radix_trie::key::KeyRef;

    #[test]
    fn test_works() {
        assert_eq!(("", "", ""), KeyRef::prefix("", ""));
        assert_eq!(("", "a", "b"), KeyRef::prefix("a", "b"));
        assert_eq!(("a", "", "b"), KeyRef::prefix("a", "ab"));
        assert_eq!(("ab", "", ""), KeyRef::prefix("ab", "ab"));
        assert_eq!(("foo", "123", "456"), KeyRef::prefix("foo123", "foo456"));
    }
}
