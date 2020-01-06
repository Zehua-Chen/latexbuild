pub trait Logger {
    fn create_dir(&mut self, dir: &str);
    fn run_latex(&mut self, latex: &str, bin: &str, entry: &str);
    fn error(&mut self, error: &str);
    fn output(&mut self, output: &str);
}

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

    fn output(&mut self, output: &str) {
        eprintln!("command output:\n{}", output);
    }
}
