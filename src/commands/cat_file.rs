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
    let start = bytes.iter().position(|&x| x == 0).unwrap();

    let mut decoder = ZlibDecoder::new(&bytes[start..]);
    let mut content = String::new();
    decoder.read_to_string(&mut content).unwrap();

    print!("{}", content);
}
