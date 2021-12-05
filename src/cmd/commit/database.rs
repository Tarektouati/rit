use crypto::digest::Digest;
use crypto::sha1::Sha1;
use deflate::write::ZlibEncoder;
use deflate::Compression;
use std::env::current_dir;
use std::fs;
use std::io::Write;

fn oid_from_str(string: &str) -> String {
   let mut hasher = Sha1::new();
   hasher.input_str(string);
   return hasher.result_str();
}

fn get_db_path() -> String {
   let path = current_dir().unwrap().display().to_string();
   return format!("{}/.git/objects", path);
}

fn compress_content(content: String) -> Option<Vec<u8>> {
   let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
   if let Ok(_) = encoder.write_all(content.as_bytes()) {
      return Some(encoder.finish().unwrap());
   }
   return None;
}

fn write_object(oid: &String, content: String) {
   let (object_folder, object_filename) = oid.split_at(2);
   let db_path = get_db_path();
   let folder_path = format!("{}/{}", db_path, object_folder);
   if let Ok(_) = fs::create_dir(&folder_path) {
      let mut file = fs::File::create(format!("{}/{}", &folder_path, object_filename)).unwrap();
      match compress_content(content) {
         Some(compressed_content) => {
            let copy_compressed_content: &[u8] = &compressed_content;
            file.write_all(copy_compressed_content).unwrap()
         }
         None => {}
      }
   }
}

pub fn store_file(file_content: String) -> String {
   // encode to ASCII_8BIT
   let byte_string: &[u8] = file_content.as_bytes();
   // format content to string with  "#{ object.type } #{ string.bytesize }\0#{ string }”
   let content = format!("blob {}\0{}", byte_string.len(), file_content);
   // create oid SHA1 of content
   let oid = oid_from_str(content.as_str());
   // write git object
   write_object(&oid, content);

   return oid;
}
