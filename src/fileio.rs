use std::io::Write;

pub trait FileAccessor {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
    fn write_all(&self, path: &str, content: &str) -> Result<(), std::io::Error>;
}

pub struct FileSystemAccessor;
impl FileAccessor for FileSystemAccessor {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn write_all(&self, path: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())
    }
}
