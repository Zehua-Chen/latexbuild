mod error;
pub use error::*;

mod log;
pub use log::*;

mod build_check;
pub use build_check::*;

mod project;
pub use project::*;

mod build;
pub use build::*;

mod generate;
use generate::Generate;

use std::fs::{remove_dir_all, File};
use std::io::BufWriter;
use std::path::PathBuf;

/// Wrapper for the build pipeline
pub struct LatexBuild<'a, L>
where
    L: Logger,
{
    /// Path to the config
    pub config_path: PathBuf,
    /// A mutable reference to the logger
    pub logger: &'a mut L,
}

impl<'a, L> LatexBuild<'a, L>
where
    L: Logger,
{
    /// Load a project and call `use_root_path` on it. In another word,
    /// load a project and make all the paths absolute
    pub fn load_project(&self) -> Result<Project, Error> {
        let mut root_path = self.config_path.clone();
        root_path.pop();

        let mut project = Project::load(&self.config_path).unwrap();
        project.use_root_path(&root_path);

        return Ok(project);
    }

    /// Run the build pipeline
    pub fn build(&mut self) -> Result<(), Error> {
        let project = self.load_project()?;

        match project.can_build() {
            Err(error) => match error {
                ProjectBuildError::NoEntry => {
                    self.logger.error("no entry file");
                    return Ok(());
                }
            },
            _ => {}
        }

        let mut needs_build_checker = NeedsBuildChecker::new(&project);

        while needs_build_checker.needs_build() {
            self.logger.message("building project");

            if !project.build(self.logger).unwrap() {
                self.logger.error("build stopped due to error");
                break;
            }
        }

        return Ok(());
    }

    pub fn clean(&mut self) -> Result<(), Error> {
        let project = self.load_project()?;
        self.logger.message("cleaning bin directory");

        match remove_dir_all(project.bin()) {
            Ok(_a) => {}
            Err(err) => {
                let message = format!("{}", err);
                self.logger.error(&message);
            }
        }

        return Ok(());
    }

    pub fn generate_make(&mut self) -> Result<(), Error> {
        let project = Project::load(&self.config_path).unwrap();
        let makefile = generate::Makefile::from(project);

        let mut file = self.config_path.clone();
        file.pop();
        file.push("Makefile");

        let file = File::create(file).unwrap();
        let mut file_writer = BufWriter::new(file);

        makefile.generate(&mut file_writer).unwrap();

        return Ok(());
    }
}
