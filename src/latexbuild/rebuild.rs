use super::Project;
use std::fs::{metadata, read_dir};
use std::io;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

/// Check if a rebuild is needed
///
/// # Arguments
/// - `pdf_modified`; the time at which the pdf is modified
/// - `includes`: the next level of dependencies
///
/// # Returns
///
/// Return `Ok(true)` if a rebuild is needed, `Ok(false)` if not needed
fn _needs_rebuild(pdf_modified: &SystemTime, deps: &Vec<PathBuf>) -> Result<bool, Error> {
    for dep in deps {
        if dep.is_dir() {
            let dir = read_dir(dep).unwrap();
            let mut children: Vec<PathBuf> = Vec::new();

            for entry_result in dir {
                children.push(entry_result?.path());
            }

            if _needs_rebuild(pdf_modified, &children)? {
                return Ok(true);
            }
        } else {
            let modified = dep.metadata()?.modified()?;

            if pdf_modified < &modified {
                return Ok(true);
            }
        }
    }

    return Ok(false);
}

impl Project {
    /// Check if a rebuild is needed
    ///
    /// # Arguments
    /// - `pdf_modified`; the time at which the pdf is modified
    /// - `includes`: the next level of dependencies
    ///
    /// # Returns
    ///
    /// Return `Ok(true)` if a rebuild is needed, `Ok(false)` if not needed
    pub fn needs_rebuild(&self) -> Result<bool, io::Error> {
        let pdf_metadata = metadata(&self.pdf);

        println!("pdf path = {:?}", self.pdf);

        return match pdf_metadata {
            Ok(metadata) => {
                let modified = metadata.modified().unwrap();
                let mut deps: Vec<PathBuf> = Vec::new();

                deps.push(self.entry.clone());

                for include in &self.includes {
                    deps.push(include.clone());
                }

                _needs_rebuild(&modified, &deps)
            }
            Err(_) => Ok(true),
        };
    }
}
