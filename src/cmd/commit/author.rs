use chrono::offset::Local;

pub struct Author {
    name: String,
    email: String,
    timestamp: String,
    offset: String,
}

impl Author {
    pub fn new(name: String, email: String) -> Author {
        let ln = Local::now();
        Author {
            name: name.to_string(),
            email: email.to_string(),
            timestamp: ln.timestamp().to_string(),
            offset: ln.with_timezone(&Local).format("%z").to_string(),
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "{} <{}> {} {}",
            self.name, self.email, self.timestamp, self.offset
        )
    }
}
