use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct Lockfile {
    path: PathBuf,
    file: File,
}

impl Lockfile {
    pub fn hold_for_update(path: PathBuf) -> Result<Option<Self>, ErrorKind> {
        let path = path.with_extension("lock");
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .create(true)
            .open(&path);

        let lockfile = match file {
            Ok(f) => f,
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    return Ok(None);
                } else {
                    return Err(err.kind());
                }
            }
        };

        Ok(Some(Self {
            path,
            file: lockfile,
        }))
    }

    pub fn commit(&self) -> Result<(), ()> {
        unimplemented!()
    }
}
