pub trait Logger {
    fn create_dir(&mut self, dir: &str);
    fn run_latex(&mut self, latex: &str, bin: &str, entry: &str);
    fn error(&mut self, error: &str);
    fn output(&mut self, output: &str);
}

pub struct StdIOLogger {}

impl StdIOLogger {
    pub fn new() -> StdIOLogger {
        StdIOLogger {}
    }
}

impl Logger for StdIOLogger {
    fn create_dir(&mut self, dir: &str) {
        println!("creating directory {}", dir);
    }

    fn run_latex(&mut self, latex: &str, bin: &str, entry: &str) {
        println!("running latex {}, bin = {}, entry = {}", latex, bin, entry);
    }

    fn error(&mut self, error: &str) {
        eprintln!("{}", error);
    }

    fn output(&mut self, output: &str) {
        eprintln!("command output:\n{}", output);
    }
}
