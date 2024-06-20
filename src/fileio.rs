pub trait FileAccessor {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
}

pub struct FileSystemAccessor;
impl FileAccessor for FileSystemAccessor {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}
