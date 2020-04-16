use json::object::Object;
use json::{parse, JsonValue};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::{read, read_dir};
use std::io;
use std::path::{Path, PathBuf};
use std::string;

fn resolve_includes(includes: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut to_explore: Vec<PathBuf> = Vec::new();

    for include in includes {
        if include.is_dir() {
            to_explore.push(include);
        } else {
            files.push(include);
        }
    }

    while !to_explore.is_empty() {
        let dir = to_explore.pop().unwrap();

        for dir_item in read_dir(dir).unwrap() {
            let dir_item = dir_item.unwrap().path();

            if dir_item.is_dir() {
                to_explore.push(dir_item);
                continue;
            }

            files.push(dir_item);
        }
    }

    return files;
}

/// A project loaded from disk
///
/// # Fields
///
/// - `pdf`: note that the pdf field is not in the project specification,
///   instead, it is inferred from the project specification
/// - `aux`: note that the aux field is not in the project specification,
///   instead, it is inferred from the project specification
///
/// # Discussion
///
/// A this stage of the project, none of the fields of the structure should be
/// edited by the consumer. In the future, the fields would be replaced by
/// getters and setters
pub struct Project {
    /// The `latex` program used
    latex: OsString,
    /// The output directory
    bin: PathBuf,
    /// The pdf file
    pdf: PathBuf,
    /// The aux file
    aux: PathBuf,
    /// The entry latex file
    entry: PathBuf,
    /// The include files and directories
    files: Vec<PathBuf>,
}

/// Error from loading projects
pub enum ProjectLoadError {
    /// Something wrong with the format
    FormatError,
    /// Something wrong with the parser
    ParserError(json::Error),
    /// Something wrong with IO
    IOError(io::Error),
    /// Something wrong with UTF8 conversion
    FromUtf8Error(string::FromUtf8Error),
}

impl From<json::Error> for ProjectLoadError {
    fn from(error: json::Error) -> ProjectLoadError {
        ProjectLoadError::ParserError(error)
    }
}

impl From<io::Error> for ProjectLoadError {
    fn from(error: io::Error) -> ProjectLoadError {
        ProjectLoadError::IOError(error)
    }
}

impl From<string::FromUtf8Error> for ProjectLoadError {
    fn from(error: string::FromUtf8Error) -> ProjectLoadError {
        ProjectLoadError::FromUtf8Error(error)
    }
}

impl Debug for ProjectLoadError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ProjectLoadError::FormatError => {
                f.write_str("format error")?;
            }
            _ => {
                f.write_str("other error")?;
            }
        }

        return Ok(());
    }
}

fn with_prepend(path: &PathBuf, prepend: &Path) -> PathBuf {
    let mut output = PathBuf::from(prepend);
    output.push(path);

    return output;
}

impl Project {
    /// Create a new project with the following default values
    ///
    /// - `latex`: `pdflatex`
    /// - `bin`: `bin`
    /// - `pdf`: `index.pdf`
    /// - `entry`: `index.tex`
    /// - `includes`: []
    pub fn new() -> Project {
        let mut files: Vec<PathBuf> = Vec::new();
        files.push(PathBuf::from("index.tex"));

        Project {
            latex: OsString::from("pdflatex"),
            bin: PathBuf::from("bin"),
            pdf: PathBuf::from("bin/index.pdf"),
            aux: PathBuf::from("bin/index.aux"),
            entry: PathBuf::from("index.tex"),
            files,
        }
    }

    /// Load a project from a json path
    ///
    /// # Arguments
    ///
    /// - `path`: the path to the json
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Project, ProjectLoadError> {
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
                            return Err(ProjectLoadError::FormatError);
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
                            return Err(ProjectLoadError::FormatError);
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
                            return Err(ProjectLoadError::FormatError);
                        }
                    },
                    _ => {}
                }

                // After getting entry, resolve
                // - pdf
                // - aux
                project.pdf = project.bin.clone();
                project.pdf.push(&project.entry);
                project.pdf.set_extension("pdf");

                project.aux = project.bin.clone();
                project.aux.push(&project.entry);
                project.aux.set_extension("aux");

                // includes
                match object.get("includes") {
                    Some(includes) => match includes {
                        JsonValue::Array(includes_array) => {
                            for include in includes_array {
                                match include {
                                    JsonValue::Short(include_short) => {
                                        project.files.push(PathBuf::from(include_short.as_str()));
                                    }
                                    JsonValue::String(include_str) => {
                                        project.files.push(PathBuf::from(include_str));
                                    }
                                    _ => {
                                        return Err(ProjectLoadError::FormatError);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ProjectLoadError::FormatError);
                        }
                    },
                    _ => {}
                }

                project.files.push(PathBuf::from("index.tex"));
                project.files = resolve_includes(project.files);

                Ok(project)
            }
            _ => Err(ProjectLoadError::FormatError),
        };
    }

    pub fn pdf(&self) -> &Path {
        return &self.pdf;
    }

    pub fn aux(&self) -> &Path {
        return &self.aux;
    }

    pub fn bin(&self) -> &Path {
        return &self.bin;
    }

    pub fn latex(&self) -> &OsStr {
        return &self.latex;
    }

    pub fn entry(&self) -> &Path {
        return &self.entry;
    }

    pub fn files(&self) -> &Vec<PathBuf> {
        return &self.files;
    }

    /// Use a root path
    ///
    /// # Arguments
    ///
    /// - `root_path`: the root path
    pub fn use_root_path(&mut self, root_path: &Path) {
        // bin
        self.bin = with_prepend(&self.bin, root_path);

        // entry
        self.entry = with_prepend(&self.entry, root_path);

        // includes
        let mut files: Vec<PathBuf> = Vec::new();

        for file in &self.files {
            files.push(with_prepend(file, root_path));
        }

        self.files = files;

        // pdf
        self.pdf = with_prepend(&self.pdf, root_path);

        // aux
        self.aux = with_prepend(&self.aux, root_path);
    }
}

impl Into<JsonValue> for Project {
    fn into(self) -> JsonValue {
        let default = Project::new();
        let mut object = Object::new();

        object.insert(
            "latex",
            JsonValue::String(String::from(default.latex().to_str().unwrap())),
        );
        object.insert(
            "bin",
            JsonValue::String(String::from(default.bin().to_str().unwrap())),
        );
        object.insert(
            "entry",
            JsonValue::String(String::from(default.entry().to_str().unwrap())),
        );

        object.insert("includes", JsonValue::new_array());

        return JsonValue::Object(object);
    }
}
