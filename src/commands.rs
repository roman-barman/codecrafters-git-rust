mod cat_file;
mod init;

pub(crate) use cat_file::cat_file;
pub(crate) use init::init;

const OBJECT_DIR: &str = ".git/objects";
