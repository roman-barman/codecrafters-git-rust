use crate::commands::OBJECT_DIR;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;

pub(crate) fn cat_file(hash: &str) {
    let dir_name = &hash[..2];
    let file_name = &hash[2..];

    let bytes =
        fs::read(format!("./{}/{}/{}", OBJECT_DIR, dir_name, file_name)).unwrap_or_else(|_| {
            panic!(
                "Failed to read file: {}/{}/{}",
                OBJECT_DIR, dir_name, file_name
            )
        });

    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut content = String::new();
    decoder.read_to_string(&mut content).expect("Failed to read zlib compressed content");
    let content_start = content.find("\0").unwrap_or(0);

    print!("{}", &content.as_str()[content_start..]);
}
