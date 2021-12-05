use std::env::current_dir;
use std::fs;
use walkdir::{DirEntry, WalkDir};
mod database;

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap() == ".git";
}

fn read_file(file_path: &String) -> String {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    content
}

pub fn create_commit() {
    let path = current_dir().unwrap().display().to_string();
    let files: Vec<_> = WalkDir::new(&path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| !is_git_dir(f))
        .filter(|f| f.file_type().is_file())
        .map(|f| f.path().to_owned())
        .collect();
    
    let entries: Vec<(String, String)> = files
        .iter()
        .map(|f| {
            let file_path: String = f.display().to_string();
            let content = read_file(&file_path);
            (file_path, content)
        })
        .map(|f| {
            let (file_path, content) = f;
            let oid = database::store_file(content);
            return (oid, file_path);
        })
        .collect();
        // create tree from entries
}
