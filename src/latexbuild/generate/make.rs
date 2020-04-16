use super::super::Project;
use super::Generate;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub enum MakeDependency {
    Regular(String),
    OrderOnly(String),
}

impl Generate for MakeDependency {
    fn generate(&self, writer: &mut BufWriter<File>) -> io::Result<()> {
        match self {
            MakeDependency::Regular(ref d) => {
                write!(writer, "{}", d)?;
            }
            MakeDependency::OrderOnly(ref d) => {
                write!(writer, "| {}", d)?;
            }
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
    fn generate(&self, writer: &mut BufWriter<File>) -> io::Result<()> {
        for target in &self.targets {
            write!(writer, "{}: ", target.target)?;

            let mut dep_iter = target.dependencies.iter();

            match dep_iter.next() {
                Some(dep) => {
                    dep.generate(writer)?;
                }
                _ => {},
            }

            for dep in dep_iter {
                write!(writer, " ")?;
                dep.generate(writer)?;
            }

            writeln!(writer)?;
            write!(writer, "\t")?;

            writeln!(writer, "{}", target.command)?;
        }

        return Ok(());
    }
}

impl From<Project> for Makefile {
    fn from(project: Project) -> Makefile {
        let mut dependencies: Vec<MakeDependency> = Vec::new();

        for file in project.files() {
            dependencies.push(MakeDependency::Regular(String::from(
                file.to_str().unwrap(),
            )));
        }

        dependencies.push(MakeDependency::OrderOnly(String::from(
            project.bin().to_str().unwrap(),
        )));

        let mut makefile = Makefile::new();

        makefile.targets.push(MakeTarget {
            target: String::from(project.pdf().to_str().unwrap()),
            command: format!(
                "{} -output-directory={} {}",
                project.latex().to_str().unwrap(),
                project.bin().to_str().unwrap(),
                project.entry().to_str().unwrap()),
            dependencies: dependencies,
        });

        makefile.targets.push(MakeTarget {
            target: String::from(project.bin().to_str().unwrap()),
            command: format!("mkdir {}", project.bin().to_str().unwrap()),
            dependencies: Vec::new()
        });

        return makefile;
    }
}
