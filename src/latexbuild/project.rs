use super::Error;
use json::object::Object;
use json::{parse, JsonValue};
use std::ffi::{OsStr, OsString};
use std::fs::{read, read_dir};
use std::path::{Path, PathBuf};

fn resolve_includes(includes: Vec<PathBuf>) -> Result<Vec<PathBuf>, Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut to_explore: Vec<PathBuf> = Vec::new();

    for include in includes {
        if include.is_dir() {
            to_explore.push(include);
        } else {
            files.push(include);
        }
    }

    while let Some(dir) = to_explore.pop() {
        match read_dir(&dir) {
            Ok(dir) => {
                for dir_item in dir {
                    match dir_item {
                        Ok(dir_item) => {
                            let dir_item = dir_item.path();

                            if dir_item.is_dir() {
                                to_explore.push(dir_item);
                            } else {
                                files.push(dir_item);
                            }
                        }
                        Err(error) => return Err(Error::IO(error)),
                    }
                }
            }
            Err(_) => return Err(Error::PathNotFound(dir)),
        }
    }

    return Ok(files);
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
    pub fn load<P: AsRef<Path>>(path: &P) -> Result<Project, Error> {
        let mut project = Project::new();
        let file_content: String;

        match read(path) {
            Ok(raw_content) => {
                file_content = match String::from_utf8(raw_content) {
                    Ok(s) => s,
                    Err(_) => return Err(Error::Encoding),
                };
            }
            Err(_) => {
                return Err(Error::PathNotFound(PathBuf::from(path.as_ref())));
            }
        }

        if file_content.is_empty() {
            return Ok(project);
        }

        let parsed = match parse(&file_content) {
            Ok(json_value) => json_value,
            Err(error) => return Err(Error::JsonParsing(error)),
        };

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
                            return Err(Error::WrongConfigFormat(String::from(
                                "\"latex\" should be a string",
                            )));
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
                            return Err(Error::WrongConfigFormat(String::from(
                                "\"bin\" should be a string",
                            )));
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
                            return Err(Error::WrongConfigFormat(String::from(
                                "\"entry\" should be a string",
                            )));
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
                                        return Err(Error::WrongConfigFormat(String::from(
                                            "items in \"includes\" strings",
                                        )));
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(Error::WrongConfigFormat(String::from(
                                "\"includes\" should be an array",
                            )));
                        }
                    },
                    _ => {}
                }

                project.files.push(project.entry.clone());
                project.files = resolve_includes(project.files)?;

                Ok(project)
            }
            _ => Err(Error::WrongConfigFormat(String::from("expecting object"))),
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
        let mut object = Object::new();

        match self.latex.to_str() {
            Some(s) => {
                object.insert("latex", JsonValue::String(String::from(s)));
            }
            None => {
                object.insert("latex", JsonValue::String(String::from("pdflatex")));
            }
        }

        match self.bin.to_str() {
            Some(s) => {
                object.insert("bin", JsonValue::String(String::from(s)));
            }
            None => {
                object.insert("bin", JsonValue::String(String::from("bin")));
            }
        }

        match self.entry.to_str() {
            Some(s) => {
                object.insert("entry", JsonValue::String(String::from(s)));
            }
            None => {
                object.insert("entry", JsonValue::String(String::from("index.tex")));
            }
        }

        let mut includes: Vec<JsonValue> = Vec::new();

        for file in self.files {
            match file.to_str() {
                Some(s) => includes.push(JsonValue::String(String::from(s))),
                _ => {}
            }
        }

        object.insert("includes", JsonValue::Array(includes));

        return JsonValue::Object(object);
    }
}
