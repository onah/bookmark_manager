pub trait ProcessExecutor {
    fn execute(&self, command: &str, arg: &str) -> Result<(), std::io::Error>;
}

pub struct ProcessExecutorImpl;
impl ProcessExecutor for ProcessExecutorImpl {
    fn execute(&self, command: &str, arg: &str) -> Result<(), std::io::Error> {
        let _output = std::process::Command::new(command).arg(arg).output()?;
        Ok(())
    }
}
