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
