use super::{Logger, Project};
use std::fs::create_dir;
use std::io;
use std::process::Command;

pub enum ProjectBuildError {
    NoEntry,
}

impl Project {
    pub fn build<L: Logger>(&self, logger: &mut L) -> io::Result<()> {
        if !self.bin.exists() {
            logger.create_dir(self.bin.to_str().unwrap());
            create_dir(&self.bin)?;
        }

        let output_dir_arg = format!("-output-directory={}", self.bin.to_str().unwrap());

        logger.run_latex(
            self.latex.to_str().unwrap(),
            self.bin.to_str().unwrap(),
            self.entry.to_str().unwrap(),
        );

        let command_output = Command::new(&self.latex)
            .args(&[&output_dir_arg, self.entry.to_str().unwrap()])
            .output()?;

        let command_output_str = String::from_utf8(command_output.stdout).unwrap();

        logger.output(&command_output_str);

        return Ok(());
    }

    pub fn can_build(&self) -> Result<(), ProjectBuildError> {
        if !self.entry.exists() {
            return Err(ProjectBuildError::NoEntry);
        }

        return Ok(());
    }
}
