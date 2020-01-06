/// Logger
pub trait Logger {
    /// Called when a directory is created
    fn create_dir(&mut self, dir: &str);
    /// Called when the latex command is run
    fn run_latex(&mut self, latex: &str, bin: &str, entry: &str);
    /// Called when a error occurs
    fn error(&mut self, error: &str);
    /// Called when an output is produced from the latex subcommand
    fn latex_output(&mut self, output: &str);
}

/// `trait Logger` implementation for standard error
pub struct StdErrLogger {}

impl StdErrLogger {
    pub fn new() -> StdErrLogger {
        StdErrLogger {}
    }
}

impl Logger for StdErrLogger {
    fn create_dir(&mut self, dir: &str) {
        eprintln!("creating directory {}", dir);
    }

    fn run_latex(&mut self, latex: &str, bin: &str, entry: &str) {
        eprintln!("running latex {}, bin = {}, entry = {}", latex, bin, entry);
    }

    fn error(&mut self, error: &str) {
        eprintln!("{}", error);
    }

    fn latex_output(&mut self, output: &str) {
        eprintln!("command output:\n{}", output);
    }
}
