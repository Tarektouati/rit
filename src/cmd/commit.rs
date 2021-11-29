use std::env::current_dir;
use std::fs;
use walkdir::{DirEntry, WalkDir};
mod database;


fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap() == ".git";
}

fn read_file(entry: &DirEntry) -> Result<String, std::io::Error> {
    let file_path = entry.path().display().to_string();
    if entry.file_type().is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Cannot read directory content, try a file instead",
        ));
    }
    let content = fs::read_to_string(file_path);
    match content {
        Ok(content) => Ok(content),
        Err(err) => Err(err),
    }
}

pub fn create_commit() {
    let path = current_dir().unwrap().display().to_string();
    let mut walker = WalkDir::new(&path).into_iter();
    loop {
        match walker.next() {
            Some(Ok(entry)) => {
                if is_git_dir(&entry) {
                    return;
                }
                if let Ok(file_content) = read_file(&entry) {
                    database::store_file(file_content)
                }
            }
            Some(Err(e)) => println!("Error: {}", e),
            None => break,
        }
    }
}
