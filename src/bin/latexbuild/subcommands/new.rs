use super::*;

pub fn new(matches: &ArgMatches) {
    let name = matches.value_of("name").unwrap();
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
