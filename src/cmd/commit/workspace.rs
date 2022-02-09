use walkdir::{DirEntry, WalkDir};
use std::fs;
use std::path::PathBuf;

pub struct Workspace {
    path: String,
 }

impl Workspace {
    pub fn new(path: &String) -> Workspace {
        Workspace {
            path: path.to_string(),
        }
    }
    pub fn list_files(&self) -> Vec<PathBuf> {
        return WalkDir::new(&self.path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| !self.is_git_dir(f))
        .filter(|f| f.file_type().is_file())
        .map(|f| f.path().to_owned())
        .collect();
    }

    pub fn read_file(&self, file: &PathBuf) -> (String, String) {
        let shortend_path = format!("{}/", &self.path);
        let file_path: String = file.display().to_string();
        let file_name: String = String::from(&file_path.replace(&shortend_path, ""));
        let content = fs::read_to_string(&file_path).expect(format!("Something went wrong reading the file: {}", &file_path).as_str());
        (file_name, content)
    }

    fn is_git_dir(&self, entry: &DirEntry) -> bool {
        return entry.path().to_str().unwrap().contains(".git");
    }
}
