use super::Project;
use std::fs::{metadata, read, read_dir};
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
fn needs_build(modified: &SystemTime, deps: &Vec<PathBuf>) -> Result<bool, Error> {
    for dep in deps {
        if dep.is_dir() {
            let dir = read_dir(dep).unwrap();
            let mut children: Vec<PathBuf> = Vec::new();

            for entry_result in dir {
                children.push(entry_result?.path());
            }

            if needs_build(modified, &children)? {
                return Ok(true);
            }
        } else {
            let dep_modified = dep.metadata()?.modified()?;

            if modified <= &dep_modified {
                return Ok(true);
            }
        }
    }

    return Ok(false);
}

/// An object that determine if a build is still needed
///
/// # Discussion
///
/// - `should_build()` method must be called before calling the `build()` method
///   of project
pub struct NeedsBuildChecker<'a> {
    pub project: &'a Project,
    old_aux: Option<Vec<u8>>,
    has_checked_sources: bool,
    has_checked_aux: bool,
}

impl<'a> NeedsBuildChecker<'a> {
    /// Create a needs build checker using a project
    ///
    /// # Arguments
    ///
    /// -
    pub fn new(project: &'a Project) -> NeedsBuildChecker<'a> {
        let old_aux: Option<Vec<u8>> = match read(project.aux()) {
            Ok(aux) => Some(aux),
            _ => None,
        };

        NeedsBuildChecker {
            project: project,
            old_aux: old_aux,
            has_checked_sources: false,
            has_checked_aux: false,
        }
    }

    /// Determine if a build is needed
    ///
    /// # Returns
    ///
    /// `true` is a build is needed
    pub fn needs_build(&mut self) -> bool {
        if !self.has_checked_sources {
            self.has_checked_sources = true;
            let pdf_metadata = metadata(self.project.pdf());

            match pdf_metadata {
                // if pdf exists, check to see of sources are newer than pdf
                // if yes, rebuild
                Ok(metadata) => {
                    let mut deps: Vec<PathBuf> = Vec::new();

                    deps.push(PathBuf::from(self.project.entry()));

                    for include in self.project.includes() {
                        deps.push(include.clone());
                    }

                    let modified = metadata.modified().unwrap();

                    return needs_build(&modified, &deps).unwrap();
                }
                // if pdf does not exist, rebuild
                Err(_) => {
                    return true;
                }
            };
        }

        if !self.has_checked_aux {
            self.has_checked_aux = true;

            match &self.old_aux {
                // originally have an aux file
                Some(old_aux) => {
                    match read(self.project.aux()) {
                        // Cannot open the new aux file, so return false
                        // to be safe
                        Err(_) => {
                            return false;
                        }
                        Ok(ref new_aux) => return !(new_aux == old_aux),
                    }
                }
                // we did not originally have a aux file, it means the project
                // has never been built, therefore, needs a build
                None => {
                    return true;
                }
            }
        }

        return false;
    }
}
