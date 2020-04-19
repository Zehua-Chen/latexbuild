use super::super::Project;
use super::Error;
use super::Generate;
use std::fs::File;
use std::io::{BufWriter, Write};

pub enum MakeDependency {
    Regular(String),
    OrderOnly(String),
}

impl Generate for MakeDependency {
    fn generate(&self, writer: &mut BufWriter<File>) -> Result<(), Error> {
        match self {
            MakeDependency::Regular(ref d) => match write!(writer, "{}", d) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            },
            MakeDependency::OrderOnly(ref d) => match write!(writer, "| {}", d) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            },
        }

        return Ok(());
    }
}

pub struct MakeTarget {
    target: String,
    command: String,
    dependencies: Vec<MakeDependency>,
}

pub struct Makefile {
    targets: Vec<MakeTarget>,
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            targets: Vec::new(),
        }
    }
}

impl Generate for Makefile {
    fn generate(&self, writer: &mut BufWriter<File>) -> Result<(), Error> {
        for target in &self.targets {
            match write!(writer, "{}: ", target.target) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            }

            let mut dep_iter = target.dependencies.iter();

            match dep_iter.next() {
                Some(dep) => {
                    dep.generate(writer)?;
                }
                _ => {}
            }

            for dep in dep_iter {
                match write!(writer, " ") {
                    Err(error) => return Err(Error::IO(error)),
                    _ => {}
                }

                dep.generate(writer)?;
            }

            match writeln!(writer) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            }

            match write!(writer, "\t") {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            }

            match writeln!(writer, "{}", target.command) {
                Err(error) => return Err(Error::IO(error)),
                _ => {}
            }
        }

        return Ok(());
    }
}

impl Project {
    pub fn to_make(&self) -> Result<Makefile, Error> {
        let mut dependencies: Vec<MakeDependency> = Vec::new();

        for file in self.files() {
            match file.to_str() {
                Some(file) => {
                    dependencies.push(MakeDependency::Regular(String::from(file)));
                }
                _ => {}
            }
        }

        let project = match self.bin().to_str() {
            Some(project) => project,
            _ => return Err(Error::Encoding),
        };

        dependencies.push(MakeDependency::OrderOnly(String::from(project)));

        let mut makefile = Makefile::new();

        let latex = match self.latex().to_str() {
            Some(latex) => latex,
            _ => return Err(Error::Encoding),
        };

        let bin = match self.bin().to_str() {
            Some(bin) => bin,
            _ => return Err(Error::Encoding),
        };

        let pdf = match self.pdf().to_str() {
            Some(pdf) => pdf,
            _ => return Err(Error::Encoding),
        };

        let entry = match self.entry().to_str() {
            Some(entry) => entry,
            _ => return Err(Error::Encoding),
        };

        makefile.targets.push(MakeTarget {
            target: String::from(pdf),
            command: format!("{} -output-directory={} {}", latex, bin, entry),
            dependencies: dependencies,
        });

        makefile.targets.push(MakeTarget {
            target: String::from(bin),
            command: format!("mkdir {}", bin),
            dependencies: Vec::new(),
        });

        return Ok(makefile);
    }
}
