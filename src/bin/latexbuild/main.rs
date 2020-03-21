use clap::{App, Arg, SubCommand};
use latexbuild::*;
use std::fs::{create_dir, write};
use std::path::PathBuf;

mod subcommands;

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
            subcommands::clean(clean_m);
        }
        ("new", Some(new_m)) => {
            subcommands::new(new_m);
        }
        _ => {
            subcommands::build(&matches);
        }
    };
}
