use clap::{App, Arg, SubCommand};
use latexbuild::*;
use std::fs::{create_dir, write};
use std::path::PathBuf;

const ENTRY_STR: &'static str = "\\documentclass{article}

\\begin{document}
  Hello LaTeX
\\end{document}
";

fn main() {
    let matches = App::new("latexbuild")
        .version("0.1")
        .author("Zehua Chen peterchen06242000@outlook.com")
        .about("A tool to build latex projects")
        .args(&[Arg::with_name("config")
            .short("c")
            .long("config")
            .default_value("./latexproject.json")
            .help("Path to the configuration file")])
        .subcommand(
            SubCommand::with_name("clean")
                .args(&[Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .help("Path to the configuration file")
                    .default_value("./latexproject.json")])
                .about("Clean build directory"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .args(&[Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("Name of the project")
                    .required(true)
                    .index(1)])
                .about("Create a new project"),
        )
        .get_matches();

    match matches.subcommand() {
        ("clean", Some(clean_m)) => {
            let mut logger = StdErrLogger::new();
            let mut latexbuild = LatexBuild {
                config_path: PathBuf::from(clean_m.value_of("config").unwrap()),
                logger: &mut logger,
            };

            latexbuild.clean();
        }
        ("new", Some(new_m)) => {
            let name = new_m.value_of("name").unwrap();
            let project_root = PathBuf::from(name);
            let mut logger = StdErrLogger::new();

            // Create project dir
            logger.message("creating project directory");
            create_dir(&project_root).unwrap();

            // Create config file
            logger.message("creating config file");
            let mut config_path = project_root.clone();
            config_path.push("latexproject");
            config_path.set_extension("json");

            let config_str = json::stringify_pretty(Project::new(), 2);

            write(&config_path, config_str.as_bytes()).unwrap();

            // Create entry file
            logger.message("creating entry file");
            let mut entry_path = project_root.clone();
            entry_path.push("index");
            entry_path.set_extension("tex");

            write(&entry_path, ENTRY_STR.as_bytes()).unwrap();

            logger.message("done");
        }
        _ => {
            let mut logger = StdErrLogger::new();
            let mut latexbuild = LatexBuild {
                config_path: PathBuf::from(matches.value_of("config").unwrap()),
                logger: &mut logger,
            };

            latexbuild.build();
        }
    };
}
