const MODE: &str = "100644";

pub struct Tree {
    entries: Vec<(String, String)>
}

impl Tree {
    pub fn new(entries: Vec<(String, String)>) -> Tree {
        Tree { entries: entries }
    }

    fn oid_to_ascii_bytes(&self, oid: String) -> Vec<String> {
        return oid
        .as_bytes()
        .chunks(2)
         .map(|bytes| bytes[0] + bytes[1])
         .map(|b| format!("{:x}", b))
         .map(|s| u8::from_str_radix(&s, 16).map(|n| n as char).unwrap())
         .map(|c| format!("{}", c))
         .collect::<Vec<String>>();
    }

    pub fn to_string(&self) -> String {
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort();
        let tree: Vec<String> = sorted_entries
           .iter()
           .map(|(oid, name)|  format!("{} {};{}", MODE, name, self.oid_to_ascii_bytes(oid.to_string()).join("")))
           .collect();
        return tree.join("")
    }
}
