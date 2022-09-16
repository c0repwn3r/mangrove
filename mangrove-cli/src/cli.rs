use std::error::Error;

pub trait ExecutableCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}