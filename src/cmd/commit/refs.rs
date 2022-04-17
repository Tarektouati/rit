use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use super::lockfile::Lockfile;

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
                eprintln!("{}", e);
                None
            }
        }
    }
    pub fn write(&self, oid: &String) -> Result<(), Error> {
        if let Some(mut file) = Lockfile::hold_for_update(Path::new(&self.head_path).to_path_buf())? {
            file.write_all(oid.as_bytes())?;
            file.write_all(b"\n")?;
            file.commit()?;
        }
        return Ok(());

    }
}
