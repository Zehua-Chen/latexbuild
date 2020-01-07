use std::ffi::OsStr;

/// Logger
pub trait Logger {
    /// Called when a directory is created
    fn create_dir<S>(&mut self, dir: S) where S: AsRef<str>;
    /// Called when a command is run
    ///
    /// # Parameters
    ///
    /// - `command`: the comamnd string
    /// - `args`: an iterator to the arguments used
    fn run_command<CS, I, S>(&mut self, command: CS, args: I)
    where
        CS: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
    /// Called when a command has produced outputs
    ///
    /// # Parameter
    ///
    /// - `s`: the command output
    fn command_output<S>(&mut self, s: S)
    where
        S: AsRef<str>;
    /// Called when a error occurs
    ///
    /// # Parameter
    ///
    /// - `error`: the error string
    fn error<S>(&mut self, error: S) where S: AsRef<str>;
    /// Called when a message is produced
    ///
    /// # Paramaeter
    ///
    /// - `message`: the message string
    fn message<S>(&mut self, message: S) where S: AsRef<str>;
}

/// `trait Logger` implementation for standard error
pub struct StdErrLogger {}

impl StdErrLogger {
    pub fn new() -> StdErrLogger {
        StdErrLogger {}
    }
}

impl Logger for StdErrLogger {
    fn create_dir<S>(&mut self, dir: S) where S: AsRef<str> {
        eprintln!("creating directory {}", dir.as_ref());
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

    fn error<S>(&mut self, error: S) where S: AsRef<str> {
        eprintln!("{}", error.as_ref());
    }

    fn message<S>(&mut self, message: S) where S: AsRef<str> {
        eprintln!("{}", message.as_ref());
    }
}
