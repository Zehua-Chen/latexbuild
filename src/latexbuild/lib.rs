mod io;
pub use io::*;

mod rebuild;
pub use rebuild::*;

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
    fn _load_project(&self) -> Project {
        let mut root_path = self.config_path.clone();
        root_path.pop();

        let mut project = Project::load(&self.config_path).unwrap();
        project.use_root_path(&root_path);

        return project;
    }

    /// Run the build pipeline
    pub fn build(&mut self) {
        let project = self._load_project();

        match project.can_build() {
            Err(error) => match error {
                ProjectBuildError::NoEntry => {
                    println!("no entry file");
                    return;
                }
            },
            _ => {}
        }

        match project.needs_rebuild() {
            Ok(needs_rebuild) => {
                if needs_rebuild {
                    project.build(self.logger).unwrap();
                } else {
                    println!("no rebuild needed");
                }
            }
            Err(_) => {
                return;
            }
        }
    }

    pub fn clean(&mut self) {
        let project = self._load_project();

        match remove_dir_all(project.bin) {
            Ok(_a) => {}
            Err(err) => {
                let message = format!("{}", err);
                self.logger.error(&message);
            }
        }
    }
}
