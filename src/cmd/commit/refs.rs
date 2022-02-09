use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

pub struct Refs {
    head_path: String,
}

impl Refs {
    pub fn new(pathname: &String) -> Refs {
        Refs {
            head_path: format!("{}/.git/HEAD", pathname.to_string()),
        }
    }

    pub fn read(&self) -> Option<String> {
        let head_file = Path::new(&self.head_path);
        if !head_file.exists() {
            return None;
        }
        match File::open(&self.head_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                Some(contents)
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
    pub fn write(&self, oid: &String) -> Result<(), Error> {
        let mut file = File::create(&self.head_path)?;
        file.write_all(oid.as_bytes())?;
        return Ok(());
    }
}
