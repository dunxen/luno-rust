#[derive(Debug, PartialEq)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
}

impl Credentials {
    pub fn new<T: AsRef<str>>(key: T, secret: T) -> Credentials {
        Credentials {
            key: key.as_ref().into(),
            secret: secret.as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let (key, secret) = (String::from("TEST_KEY"), String::from("TEST_SECRET"));
        assert_eq!(
            super::Credentials::new(&key, &secret),
            super::Credentials { key, secret },
        )
    }
}
