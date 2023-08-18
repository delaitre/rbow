pub trait Stylize {
    fn stylize(&self, s: &str) -> Option<String>;
}

impl<T> Stylize for [T]
where
    T: Stylize,
{
    fn stylize(&self, s: &str) -> Option<String> {
        for rule in self {
            if let Some(stylized) = rule.stylize(s) {
                return Some(stylized);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock<'a> {
        s: &'a str,
        tag: &'a str,
    }

    impl Stylize for Mock<'_> {
        fn stylize(&self, s: &str) -> Option<String> {
            if self.s == s {
                return Some(self.tag.to_string());
            }

            None
        }
    }

    #[test]
    fn test_stylise_should_fail_if_no_match() {
        let v: Vec<Mock> = vec![Mock { s: "a", tag: "1" }];
        assert_eq!(v.stylize("5"), None);
    }

    #[test]
    fn test_stylise_should_return_first_match() {
        let v: Vec<Mock> = vec![
            Mock { s: "a", tag: "0" },
            Mock { s: "b", tag: "1" },
            Mock { s: "b", tag: "2" },
            Mock { s: "c", tag: "3" },
        ];
        assert_eq!(v.stylize("a"), Some("0".to_string()));
        assert_eq!(v.stylize("b"), Some("1".to_string()));
        assert_eq!(v.stylize("c"), Some("3".to_string()));
    }
}
