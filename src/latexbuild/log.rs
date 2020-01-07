use std::ffi::OsStr;

/// Logger
pub trait Logger {
    /// Called when a directory is created
    fn create_dir(&mut self, dir: &str);
    fn run_command<CS, I, S>(&mut self, command: CS, args: I)
    where
        CS: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
    fn command_output<S>(&mut self, s: S)
    where
        S: AsRef<str>;
    /// Called when a error occurs
    fn error(&mut self, error: &str);
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

    fn run_command<CS, I, S>(&mut self, command: CS, args: I)
    where
        CS: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        eprint!("\n{} ", command.as_ref().to_str().unwrap());

        for arg in args {
            eprint!("{} ", arg.as_ref().to_str().unwrap());
        }

        eprintln!();
    }

    fn command_output<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        eprintln!("\n{}", s.as_ref());
    }

    fn error(&mut self, error: &str) {
        eprintln!("{}", error);
    }
}
