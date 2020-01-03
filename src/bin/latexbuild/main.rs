use clap::{App, Arg};
use latexbuild::*;
use std::fs::metadata;
use std::path::PathBuf;

fn main() {
    let matches = App::new("latexbuild")
        .version("0.1")
        .args(&[Arg::with_name("config")
            .short("c")
            .long("config")
            .default_value("./test_project/latexproject.json")])
        .get_matches();

    let config_path = PathBuf::from(matches.value_of("config").unwrap());

    let mut root_path = config_path.clone();
    root_path.pop();

    let project = Project::load(config_path).unwrap();

    let entry = "test_project/index.tex";
    let pdf = "test_project/bin/index.pdf";

    let pdf_metadata = metadata(pdf);

    let build = match pdf_metadata {
        Ok(metadata) => {
            let modified = metadata.modified().unwrap();
            let mut deps: Vec<PathBuf> = Vec::new();
            let mut entry_path = root_path.clone();
            entry_path.push(&project.entry);

            deps.push(entry_path);

            for include in project.includes {
                let mut include_path = root_path.clone();
                include_path.push(include);

                deps.push(include_path);
            }

            needs_rebuild(&modified, &deps).unwrap()
        }
        Err(_) => true,
    };

    if build {
        println!("perform build on {}", entry);
    }
}
