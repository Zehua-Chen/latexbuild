use json::{parse, JsonValue};
use std::fs::read;
use std::io;
use std::path::{Path, PathBuf};
use std::string;
use std::fmt;
use std::fmt::{ Debug, Formatter };

pub struct Project {
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
            },
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
                // entry
                match object.get("entry") {
                    Some(entry) => match entry {
                        JsonValue::Short(entry_short) => {
                            project.entry = PathBuf::from(entry_short.as_str());
                        },
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
                                        project.includes.push(PathBuf::from(include_short.as_str()));
                                    },
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
}
