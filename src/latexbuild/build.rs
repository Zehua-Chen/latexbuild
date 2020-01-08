use super::{Logger, Project};
use std::fs::create_dir;
use std::io;
use std::process::Command;

pub enum ProjectBuildError {
    NoEntry,
}

impl Project {
    /// Build a project
    ///
    /// # Arguments
    ///
    /// - `logger`: the logger
    pub fn build<L: Logger>(&self, logger: &mut L) -> io::Result<bool> {
        if !self.bin().exists() {
            logger.message("creating bin directory");
            create_dir(self.bin())?;
        }

        let output_dir_arg = format!("-output-directory={}", self.bin().to_str().unwrap());

        logger.run_command(
            self.latex(),
            &[&output_dir_arg, self.entry().to_str().unwrap()],
        );

        let command_output = Command::new(self.latex())
            .args(&[&output_dir_arg, self.entry().to_str().unwrap()])
            .output()?;

        let command_output_str = String::from_utf8(command_output.stdout).unwrap();

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
    pub fn can_build(&self) -> Result<(), ProjectBuildError> {
        if !self.entry().exists() {
            return Err(ProjectBuildError::NoEntry);
        }

        return Ok(());
    }
}
