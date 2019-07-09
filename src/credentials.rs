pub struct Credentials {
    pub key: String,
    pub secret: String,
}

impl Credentials {
    pub fn new(key: String, secret: String) -> Credentials {
        Credentials { key, secret }
    }
}
