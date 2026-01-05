use crate::blob_object::{BlobObject, BlobObjectReadError};
use crate::commands::OBJECT_DIR;
use std::path::Path;

pub(crate) fn cat_file(object: &str) -> Result<(), BlobObjectReadError> {
    let path_string = format!("./{}", OBJECT_DIR);
    let path = Path::new(&path_string);
    let object = BlobObject::try_read(path, object)?;
    print!("{}", std::str::from_utf8(object.content()).unwrap());
    Ok(())
}
