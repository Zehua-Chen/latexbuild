mod io;
pub use io::*;

mod rebuild;
pub use rebuild::*;

mod project;
pub use project::*;

mod build;
pub use build::*;

use std::fs::metadata;
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
    /// Run the build pipeline
    pub fn run(&mut self) {
        let mut root_path = self.config_path.clone();
        root_path.pop();

        let mut project = Project::load(&self.config_path).unwrap();
        project.use_root_path(&root_path);

        match project.can_build() {
            Err(error) => match error {
                ProjectBuildError::NoEntry => {
                    println!("no entry file");
                    return;
                }
            },
            _ => {}
        }

        let pdf_metadata = metadata(&project.pdf);

        let build = match pdf_metadata {
            Ok(metadata) => {
                let modified = metadata.modified().unwrap();
                let mut deps: Vec<PathBuf> = Vec::new();

                deps.push(project.entry.clone());

                for include in &project.includes {
                    deps.push(include.clone());
                }

                needs_rebuild(&modified, &deps).unwrap()
            }
            Err(_) => true,
        };

        if build {
            project.build(self.logger).unwrap();
        }
    }
}
