use std::env::{current_dir, var};
mod author;
mod database;
mod tree;
mod workspace;

struct Commit {
    tree: String,
    author: author::Author,
    message: String,
}

impl Commit {
    pub fn new(tree: String, author: author::Author, message: String) -> Commit {
        Commit {
            tree,
            author,
            message,
        }
    }

    pub fn to_string(&self) -> String {
        let lines = [
            format!("tree {}", self.tree),
            format!("author {}", self.author.to_string()),
            format!("committer {}", self.author.to_string()),
            "".to_string(),
            self.message.to_string(),
        ];
        return lines.join("\n");
    }
}

pub fn create_commit() {
    let path = current_dir().unwrap().display().to_string();
    let database = database::Database::new(&path);
    let workspace = workspace::Workspace::new(&path);
    let entries: Vec<(String, String)> = workspace
        .list_files()
        .iter()
        .map(|f| workspace.read_file(f))
        .map(|(file_name, content)| {
            let oid = database.store(database::FileType::Blob, content);
            return (oid, file_name);
        })
        .collect();

    let tree = database.store(
        database::FileType::Tree,
        tree::Tree::new(entries).to_string(),
    );
    let name = var("RIT_AUTHOR_NAME").expect("RIT_AUTHOR_NAME not set");
    let email = var("RIT_AUTHOR_EMAIL").expect("RIT_AUTHOR_EMAIL not set");
    let author = author::Author::new(name, email);
    // TODO: ask user input commit message
    let message: &str = "first commit";
    let commit = Commit::new(tree, author, String::from(message));
    let commit_oid = database.store(database::FileType::Commit, commit.to_string());

    println!("commit: {}", commit_oid);
}
