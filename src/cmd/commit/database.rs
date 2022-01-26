use crypto::digest::Digest;
use crypto::sha1::Sha1;
use deflate::write::ZlibEncoder;
use deflate::Compression;
use std::fs::{create_dir, File};
use std::io::Write;

pub enum FileType {
   Blob,
   Tree,
   Commit,
}

pub struct Database {
   db_path: String,
}

impl Database {
   fn create_oid(&self, string: &Vec<u8>) -> String {
      let mut hasher = Sha1::new();
      hasher.input(&string);
      return hasher.result_str();
   }
   fn compress_content(&self, content: Vec<u8>) -> Option<Vec<u8>> {
      let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
      if let Ok(_) = encoder.write_all(&content) {
         return Some(encoder.finish().unwrap());
      }
      return None;
   }
   fn write_object(&self, oid: &String, content: Vec<u8>) {
      let (object_folder, object_filename) = oid.split_at(2);
      let folder_path = format!("{}/{}", self.db_path, object_folder);
      if let Ok(_) = create_dir(&folder_path) {
         let mut file = File::create(format!("{}/{}", &folder_path, object_filename)).unwrap();
         match self.compress_content(content) {
            Some(compressed_content) => {
               let copy_compressed_content: &[u8] = &compressed_content;
               file.write_all(copy_compressed_content).unwrap()
            }
            None => {}
         }
      }
   }

   pub fn new(path: &String) -> Database {
      Database {
         db_path: format!("{}/.git/objects", path.to_string()),
      }
   }

   pub fn store(&self, object_type: FileType, file_content: Vec<u8>) -> String {
      // encode to ASCII_8BIT
      // let byte_string: &[u8] = file_content
      let object_type: &str = match object_type {
         FileType::Blob => "blob",
         FileType::Tree => "tree",
         FileType::Commit => "commit",
      };
      // format content to string with  "#{ object.type } #{ string.bytesize }\0#{ string }”
      let mut content = format!("{} {}\0", object_type, file_content.len()).as_bytes().to_vec();
      content.extend_from_slice(&file_content);
      // create oid SHA1 of content
      let oid = self.create_oid(&content);
      // write git object
      self.write_object(&oid, content);
      return oid;
   }
}
