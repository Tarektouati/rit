use std::env::{current_dir, var};
use std::io::{stdin, stdout, Write};

mod author;
mod database;
mod tree;
mod workspace;
mod refs;

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

fn ask_for_commit_message() -> String {
    let mut message = String::new();
    print!("commit message: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut message).unwrap();
    return message.trim().to_string();
}

pub fn create_commit() {
    let path = current_dir().unwrap().display().to_string();
    let refs = refs::Refs::new(&path);
    let database = database::Database::new(&path);
    let workspace = workspace::Workspace::new(&path);
    let entries: Vec<(String, String)> = workspace
        .list_files()
        .iter()
        .map(|f| workspace.read_file(f))
        .map(|(file_name, content)| {
            let oid = database.store(database::FileType::Blob, content.as_bytes().to_vec());
            return (oid, file_name);
        })
        .collect();

    let tree = database.store(
        database::FileType::Tree,
        tree::Tree::new(entries).to_content(),
    );

    // let parent = refs.read();
    let name = var("RIT_AUTHOR_NAME").expect("RIT_AUTHOR_NAME not set");
    let email = var("RIT_AUTHOR_EMAIL").expect("RIT_AUTHOR_EMAIL not set");
    let author = author::Author::new(name, email);
    let message = ask_for_commit_message();
    let commit = database.store(database::FileType::Commit, Commit::new(tree, author, message).to_string().as_bytes().to_vec() );

    match refs.write(&commit){
        Ok(_) => println!("Successfully set HEAD to {}", commit),
        Err(e) => eprintln!("Failed to set HEAD to {}: {}", commit, e),
    }
}
