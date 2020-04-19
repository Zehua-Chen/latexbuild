use super::Error;
use super::Project;
use std::fs::{metadata, read};

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
    pub fn needs_build(&mut self) -> Result<bool, Error> {
        if !self.has_checked_sources {
            self.has_checked_sources = true;

            match metadata(self.project.pdf()) {
                // if pdf exists, check to see of sources are newer than pdf
                // if yes, rebuild
                Ok(metadata) => {
                    let pdf_modified = match metadata.modified() {
                        Ok(modified) => modified,
                        Err(error) => return Err(Error::IO(error)),
                    };

                    for file in self.project.files() {
                        let file_modified = match file.metadata() {
                            Ok(meta) => match meta.modified() {
                                Ok(modified) => modified,
                                Err(error) => return Err(Error::IO(error)),
                            },
                            Err(error) => return Err(Error::IO(error)),
                        };

                        if pdf_modified < file_modified {
                            return Ok(true);
                        }
                    }

                    return Ok(false);
                }
                // if pdf does not exist, rebuild
                Err(_) => {
                    return Ok(true);
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
                            return Ok(false);
                        }
                        Ok(ref new_aux) => return Ok(!(new_aux == old_aux)),
                    }
                }
                // we did not originally have a aux file, it means the project
                // has just been built for the first time, therefore, needs a
                // build
                None => {
                    return Ok(true);
                }
            }
        }

        return Ok(false);
    }
}
