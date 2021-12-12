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
   fn create_oid(&self, string: &str) -> String {
      let mut hasher = Sha1::new();
      hasher.input_str(string);
      return hasher.result_str();
   }
   fn compress_content(&self, content: String) -> Option<Vec<u8>> {
      let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
      if let Ok(_) = encoder.write_all(content.as_bytes()) {
         return Some(encoder.finish().unwrap());
      }
      return None;
   }
   fn write_object(&self, oid: &String, content: String) {
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

   pub fn store(&self, object_type: FileType, file_content: String) -> String {
      // encode to ASCII_8BIT
      let byte_string: &[u8] = file_content.as_bytes();
      let object_type: &str = match object_type {
         FileType::Blob => "blob",
         FileType::Tree => "tree",
         FileType::Commit => "commit",
      };
      // format content to string with  "#{ object.type } #{ string.bytesize }\0#{ string }”
      let content = format!("{} {}\0{}", object_type, byte_string.len(), file_content);
      // create oid SHA1 of content
      let oid = self.create_oid(content.as_str());
      // write git object
      self.write_object(&oid, content);
      return oid;
   }
}
