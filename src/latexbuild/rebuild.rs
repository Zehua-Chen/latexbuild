use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

/// Check if a rebuild is needed
///
/// # Arguments
/// - `pdf_modified`; the time at which the pdf is modified
/// - `deps`: the next level of dependencies
///
/// # Returns
///
/// Return `Ok(true)` if a rebuild is needed, `Ok(false)` if not needed
pub fn needs_rebuild(pdf_modified: &SystemTime, deps: &Vec<PathBuf>) -> Result<bool, Error> {
    for dep in deps {
        if dep.is_dir() {
            let dir = read_dir(dep).unwrap();
            let mut children: Vec<PathBuf> = Vec::new();

            for entry_result in dir {
                children.push(entry_result?.path());
            }

            if needs_rebuild(pdf_modified, &children)? {
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
