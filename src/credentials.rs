#[derive(Debug, PartialEq)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
}

impl Credentials {
    pub fn new(key: String, secret: String) -> Credentials {
        Credentials { key, secret }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let (key, secret) = (String::from("TEST_KEY"), String::from("TEST_SECRET"));
        assert_eq!(
            super::Credentials::new(key.to_owned(), secret.to_owned()),
            super::Credentials { key, secret },
        )
    }
}
