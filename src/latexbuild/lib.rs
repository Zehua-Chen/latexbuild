mod log;
pub use log::*;

mod build_check;
pub use build_check::*;

mod project;
pub use project::*;

mod build;
pub use build::*;

use std::fs::remove_dir_all;
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
    /// Load a project and call `use_root_path` on it
    fn load_project(&self) -> Project {
        let mut root_path = self.config_path.clone();
        root_path.pop();

        let mut project = Project::load(&self.config_path).unwrap();
        project.use_root_path(&root_path);

        return project;
    }

    /// Run the build pipeline
    pub fn build(&mut self) {
        let project = self.load_project();

        match project.can_build() {
            Err(error) => match error {
                ProjectBuildError::NoEntry => {
                    self.logger.error("no entry file");
                    return;
                }
            },
            _ => {}
        }

        let mut needs_build_checker = NeedsBuildChecker::new(&project);

        while needs_build_checker.needs_build() {
            self.logger.message("building project");

            if !project.build(self.logger).unwrap() {
                self.logger.error("build stopo due to error");
                break;
            }
        }
    }

    pub fn clean(&mut self) {
        let project = self.load_project();

        match remove_dir_all(project.bin()) {
            Ok(_a) => {}
            Err(err) => {
                let message = format!("{}", err);
                self.logger.error(&message);
            }
        }
    }
}
