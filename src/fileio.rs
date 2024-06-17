pub trait FileAccessor {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
}

pub struct FileSystemReader;
impl FileAccessor for FileSystemReader {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}
