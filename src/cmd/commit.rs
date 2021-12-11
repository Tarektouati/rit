use std::env::current_dir;
use std::fs;
use std::path::{PathBuf};
use walkdir::{DirEntry, WalkDir};
mod database;

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap() == ".git";
}

fn read_file(file_path: &String) -> String {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    content
}

fn prepare_file(file: &PathBuf, path: &String) -> (String, String) {
    let shortend_path =  format!("{}/", &path);
    let file_path: String = file.display().to_string();
    let file_name: String = String::from(&file_path.replace(&shortend_path, ""));
    let content = read_file(&file_path);
    (file_name, content)
}

fn find_files(path: &String) -> Vec<PathBuf> {
    return WalkDir::new(&path)
    .into_iter()
    .filter_map(|f| f.ok())
    .filter(|f| !is_git_dir(f))
    .filter(|f| f.file_type().is_file())
    .map(|f| f.path().to_owned())
    .collect();
}

pub fn create_commit() {
    let path = current_dir().unwrap().display().to_string();
    let files = find_files(&path);
    let entries: Vec<(String, String)> = files
        .iter()
        .map(|f| prepare_file(f, &path))
        .map(|(file_name, content)| {
            let oid = database::store_file(database::FileType::Blob, content);
            return (oid, file_name);
        })
        .collect();

    let tree = database::store_tree(entries);
    println!("tree: {}", tree);
    unimplemented!();
}
