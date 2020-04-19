use super::Error;
use super::{Logger, Project};
use std::fs::create_dir;
use std::process::Command;

impl Project {
    /// Build a project
    ///
    /// # Arguments
    ///
    /// - `logger`: the logger
    pub fn build<L: Logger>(&self, logger: &mut L) -> Result<bool, Error> {
        if !self.bin().exists() {
            logger.message("creating bin directory");

            match create_dir(self.bin()) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            }
        }

        let bin = match self.bin().to_str() {
            Some(s) => s,
            None => return Err(Error::Encoding),
        };

        let output_dir_arg = format!("-output-directory={}", bin);

        let entry = match self.entry().to_str() {
            Some(s) => s,
            None => return Err(Error::Encoding),
        };

        logger.run_command(self.latex(), &[&output_dir_arg, entry]);

        let command_output = match Command::new(self.latex())
            .args(&[&output_dir_arg, entry])
            .output()
        {
            Ok(output) => output,
            Err(error) => return Err(Error::IO(error)),
        };

        let command_output_str = match String::from_utf8(command_output.stdout) {
            Ok(s) => s,
            Err(_error) => return Err(Error::Encoding),
        };

        logger.command_output(&command_output_str);

        return Ok(command_output.status.success());
    }

    /// Determine if a project is buildable, it is recommended to call this
    /// method before calling the `build` method
    ///
    /// # Returns
    ///
    /// - `Ok(())` if buildable
    /// - `Err(ProjectBuildError)` if not
    pub fn can_build(&self) -> Result<(), Error> {
        if !self.entry().exists() {
            return Err(Error::NoEntry);
        }

        return Ok(());
    }
}
