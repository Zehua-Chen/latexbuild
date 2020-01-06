use clap::{App, Arg};
use latexbuild::*;
use std::path::PathBuf;

fn main() {
    let matches = App::new("latexbuild")
        .version("0.1")
        .args(&[Arg::with_name("config")
            .short("c")
            .long("config")
            .default_value("./latexproject.json")])
        .get_matches();

    let mut logger = StdErrLogger::new();
    let mut latexbuild = LatexBuild {
        config_path: PathBuf::from(matches.value_of("config").unwrap()),
        logger: &mut logger,
    };

    latexbuild.run();
}
