use clap::{App, Arg, SubCommand};
use latexbuild::*;
use std::path::PathBuf;

fn main() {
    let matches = App::new("latexbuild")
        .version("0.1")
        .args(&[Arg::with_name("config")
            .short("c")
            .long("config")
            .default_value("./latexproject.json")])
        .subcommand(
            SubCommand::with_name("clean").args(&[Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value("./latexproject.json")]),
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
