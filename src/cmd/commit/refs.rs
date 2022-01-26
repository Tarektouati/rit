use std::fs::{File};
use std::io::{Write, Error};

pub struct Refs {
    head_path: String,
}

impl Refs {
    pub fn new(pathname: &String) -> Refs {
        Refs { head_path: format!("{}/.git/HEAD", pathname.to_string()), }
    }

    pub fn read(&self) -> Result<String, Error> {
        // TODO read head 
        unimplemented!()
    }

    pub fn write(&self, oid: &String) -> Result<(), Error> {
      let mut file = File::create(&self.head_path)?;
      file.write_all(oid.as_bytes())?;
      return Ok(());
    }
}

