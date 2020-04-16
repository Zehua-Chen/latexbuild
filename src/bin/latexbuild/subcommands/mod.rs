use super::*;
use clap::ArgMatches;

mod new;
pub use new::*;

pub fn clean(matches: &ArgMatches) {
    let mut logger = StdErrLogger::new();
    let mut latexbuild = LatexBuild {
        config_path: PathBuf::from(matches.value_of("config").unwrap()),
        logger: &mut logger,
    };

    latexbuild.clean();
}

// pub fn generate(matches: &ArgMatches) {
//     let mut logger = StdErrLogger::new();
//     let mut latexbuild = LatexBuild {
//         config_path: PathBuf::from(matches.value_of("config").unwrap()),
//         logger: &mut logger,
//     };
//
//     let project = latexbuild.load_project();
// }

pub fn build(matches: &ArgMatches) {
    let mut logger = StdErrLogger::new();
    let mut latexbuild = LatexBuild {
        config_path: PathBuf::from(matches.value_of("config").unwrap()),
        logger: &mut logger,
    };

    latexbuild.build();
}

pub fn generate(matches: &ArgMatches) {
    let mut logger = StdErrLogger::new();
    let mut latexbuild = LatexBuild {
        config_path: PathBuf::from(matches.value_of("config").unwrap()),
        logger: &mut logger,
    };

    latexbuild.generate_make();
}
