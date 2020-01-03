use std::fs::{ read_dir, metadata };
use std::path::{ PathBuf };
use std::time::SystemTime;

struct Dependencies<'a> {
    modified: &'a SystemTime,
    items: Vec<PathBuf>
}

impl<'a> Dependencies<'a> {
    fn new(modified: &'a SystemTime) -> Dependencies<'a> {
        Dependencies {
            modified: modified,
            items: vec!(),
        }
    }

    fn needs_build(&self) -> bool {
        for item in &self.items {
            if item.is_dir() {
                let mut deps = Dependencies::new(self.modified);
                let dir = read_dir(item).unwrap();

                for entry_result in dir {
                    let entry = entry_result.unwrap();
                    deps.items.push(entry.path());
                }

                if deps.needs_build() {
                    return true;
                }
            } else {
                let modified = item.metadata().unwrap().modified().unwrap();

                if self.modified < &modified {
                    return true;
                }
            }
        }

        return false ;
    }
}

fn main() {
    let entry = "index.tex";
    let pdf = "bin/index.pdf";

    let pdf_metadata = metadata(pdf);

    let build = match pdf_metadata {
        Ok(metadata) => {
            let modified = metadata.modified().unwrap();

            let mut deps = Dependencies::new(&modified);
            deps.items.push(PathBuf::from("include.tex"));

            deps.needs_build()
        },
        Err(_) => {
            true
        }
    };

    if build {
        println!("perform build on {}", entry);
    }
}