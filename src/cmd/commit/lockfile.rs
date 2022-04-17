use std::fs::{rename, File, OpenOptions};
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;

pub struct Lockfile {
    path: PathBuf,
    lockfile_path: PathBuf,
    file: File,
}

impl Lockfile {
    pub fn hold_for_update(path: PathBuf) -> Result<Option<Self>, ErrorKind> {
        let lockfile_path = path.with_extension("lock");
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .create(true)
            .open(&lockfile_path);

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
            lockfile_path,
            file: lockfile,
        }))
    }

    pub fn commit(&self) -> Result<(), Error> {
        rename(&self.lockfile_path, &self.path)
    }
}

impl Write for Lockfile {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.file.flush()
    }
}
