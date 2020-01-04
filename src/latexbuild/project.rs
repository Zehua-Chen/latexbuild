use json::{parse, JsonValue};
use std::ffi::OsString;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::read;
use std::io;
use std::path::{Path, PathBuf};
use std::string;

pub struct Project {
    pub latex: OsString,
    pub bin: PathBuf,
    pub entry: PathBuf,
    pub includes: Vec<PathBuf>,
}

pub enum ProjectError {
    FormatError,
    ParserError(json::Error),
    IOError(io::Error),
    FromUtf8Error(string::FromUtf8Error),
}

impl From<json::Error> for ProjectError {
    fn from(error: json::Error) -> ProjectError {
        ProjectError::ParserError(error)
    }
}

impl From<io::Error> for ProjectError {
    fn from(error: io::Error) -> ProjectError {
        ProjectError::IOError(error)
    }
}

impl From<string::FromUtf8Error> for ProjectError {
    fn from(error: string::FromUtf8Error) -> ProjectError {
        ProjectError::FromUtf8Error(error)
    }
}

impl Debug for ProjectError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ProjectError::FormatError => {
                f.write_str("format error")?;
            }
            _ => {
                f.write_str("other error")?;
            }
        }

        return Ok(());
    }
}

impl Project {
    pub fn new() -> Project {
        Project {
            latex: OsString::from("pdflatex"),
            bin: PathBuf::from("bin"),
            entry: PathBuf::from("index.tex"),
            includes: Vec::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Project, ProjectError> {
        let mut project = Project::new();
        let file_content: String;

        match read(path) {
            Ok(raw_content) => {
                file_content = String::from_utf8(raw_content)?;
            }
            Err(_) => {
                return Ok(project);
            }
        }

        if file_content.is_empty() {
            return Ok(project);
        }

        let parsed = parse(&file_content)?;

        return match parsed {
            JsonValue::Object(object) => {
                // latex
                match object.get("latex") {
                    Some(latex) => match latex {
                        JsonValue::Short(latex_short) => {
                            project.latex = OsString::from(latex_short.as_str());
                        }
                        JsonValue::String(latex_str) => {
                            project.latex = OsString::from(latex_str);
                        }
                        _ => {
                            return Err(ProjectError::FormatError);
                        }
                    },
                    _ => {}
                }

                // bin
                match object.get("bin") {
                    Some(bin) => match bin {
                        JsonValue::Short(bin_short) => {
                            project.bin = PathBuf::from(bin_short.as_str());
                        }
                        JsonValue::String(bin_str) => {
                            project.bin = PathBuf::from(bin_str);
                        }
                        _ => {
                            return Err(ProjectError::FormatError);
                        }
                    },
                    _ => {}
                }

                // entry
                match object.get("entry") {
                    Some(entry) => match entry {
                        JsonValue::Short(entry_short) => {
                            project.entry = PathBuf::from(entry_short.as_str());
                        }
                        JsonValue::String(entry_str) => {
                            project.entry = PathBuf::from(entry_str);
                        }
                        _ => {
                            return Err(ProjectError::FormatError);
                        }
                    },
                    _ => {}
                }

                // includes
                match object.get("includes") {
                    Some(includes) => match includes {
                        JsonValue::Array(includes_array) => {
                            for include in includes_array {
                                match include {
                                    JsonValue::Short(include_short) => {
                                        project
                                            .includes
                                            .push(PathBuf::from(include_short.as_str()));
                                    }
                                    JsonValue::String(include_str) => {
                                        project.includes.push(PathBuf::from(include_str));
                                    }
                                    _ => {
                                        return Err(ProjectError::FormatError);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ProjectError::FormatError);
                        }
                    },
                    _ => {}
                }

                Ok(project)
            }
            _ => Err(ProjectError::FormatError),
        };
    }

    pub fn use_root_path(&mut self, root_path: &Path) {
        // bin
        let mut bin = PathBuf::from(root_path);
        bin.push(&self.bin);

        self.bin = bin;

        // entry
        let mut entry = PathBuf::from(root_path);
        entry.push(&self.entry);

        self.entry = entry;

        // includes
        let mut includes: Vec<PathBuf> = Vec::new();

        for include in &self.includes {
            let mut new_include = PathBuf::from(root_path);
            new_include.push(include);

            includes.push(new_include);
        }

        self.includes = includes;
    }
}
