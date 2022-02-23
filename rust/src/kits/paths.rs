use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct Paths {}

impl Paths {
    //返回可执行程序所在的路径，不包含可执行文件
    pub fn run_path() -> io::Result<PathBuf> {
        let temp = std::env::current_exe()?;
        if let Some(p) = temp.parent() {
            Ok(p.to_path_buf())
        } else {
            Err(io::Error::from(ErrorKind::NotFound))
        }
    }
}