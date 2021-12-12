use std::time::{SystemTime, UNIX_EPOCH};

pub struct Author {
    name: String,
    email: String,
    date: u128,
}

impl Author {
    pub fn new(name: String, email: String) -> Author {
        Author {
            name: name.to_string(),
            email: email.to_string(),
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} <{}> {}", self.name, self.email, self.date)
    }
}
