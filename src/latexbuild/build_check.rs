use super::Project;
use std::fs::{metadata, read_dir};
use std::io;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

/// Check if a rebuild is needed
///
/// # Arguments
/// - `modified`; the time at which the depended file is modified
/// - `includes`: the next level of dependencies
///
/// # Returns
///
/// Return `Ok(true)` if a rebuild is needed, `Ok(false)` if not needed
fn _needs_rebuild(modified: &SystemTime, deps: &Vec<PathBuf>) -> Result<bool, Error> {
    for dep in deps {
        if dep.is_dir() {
            let dir = read_dir(dep).unwrap();
            let mut children: Vec<PathBuf> = Vec::new();

            for entry_result in dir {
                children.push(entry_result?.path());
            }

            if _needs_rebuild(modified, &children)? {
                return Ok(true);
            }
        } else {
            let dep_modified = dep.metadata()?.modified()?;

            if modified < &dep_modified {
                return Ok(true);
            }
        }
    }

    return Ok(false);
}

impl Project {
    /// Check if a rebuild is needed
    ///
    /// # Returns
    ///
    /// Return `Ok(true)` if a rebuild is needed, `Ok(false)` if not needed
    pub fn needs_build(&self) -> Result<bool, io::Error> {
        let pdf_metadata = metadata(self.pdf());

        return match pdf_metadata {
            Ok(metadata) => {
                let modified = metadata.modified().unwrap();
                let mut deps: Vec<PathBuf> = Vec::new();

                deps.push(PathBuf::from(self.entry()));

                for include in self.includes() {
                    deps.push(include.clone());
                }

                _needs_rebuild(&modified, &deps)
            }
            Err(_) => Ok(true),
        };
    }
}
