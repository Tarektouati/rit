const MODE: &str = "100644";

pub struct Tree {
    entries: Vec<(String, String)>
}

impl Tree {
    pub fn new(entries: Vec<(String, String)>) -> Tree {
        Tree { entries: entries }
    }

    pub fn to_string(&self) -> String {
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort();
        let tree: Vec<_> = sorted_entries
           .iter()
           .map(|(oid, name)| format!("{} {};{:?}", MODE, name, hex::encode(oid)))
           .collect();
         tree.join("")
    }
}
