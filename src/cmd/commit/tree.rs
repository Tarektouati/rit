use std::num::ParseIntError;

const MODE: &str = "100644";


fn decode_hex(s: String) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub struct Tree {
    entries: Vec<(String, String)>
}

impl Tree {
    pub fn new(entries: Vec<(String, String)>) -> Tree {
        Tree { entries: entries }
    }

    pub fn to_content(&self) -> Vec<u8> {
        let string = self.to_string();
        let mut content : Vec<u8> =  Vec::new();
        content.extend_from_slice(&string);
        content
    }

    pub fn to_string(&self) -> Vec<u8> {
        let mut tree_vec = Vec::new();
        for (oid, name) in self.entries.iter() {
            let mut entry_vec : Vec<u8> = format!("{} {}\0", MODE, name).as_bytes().to_vec();
            println!("{}", oid);
            entry_vec.extend_from_slice(&decode_hex(oid.to_string()).expect("invalid oid"));
            tree_vec.extend_from_slice(&entry_vec);
        }
        tree_vec
    }
}
