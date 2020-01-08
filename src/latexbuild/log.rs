use ansi_term::{Color, Style};
use std::ffi::OsStr;

/// Logger
pub trait Logger {
    /// Called when a directory is created
    fn create_dir<S>(&mut self, dir: S)
    where
        S: AsRef<str>;
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
    fn error<S>(&mut self, error: S)
    where
        S: AsRef<str>;
    /// Called when a message is produced
    ///
    /// # Paramaeter
    ///
    /// - `message`: the message string
    fn message<S>(&mut self, message: S)
    where
        S: AsRef<str>;
}

/// `trait Logger` implementation for standard error
pub struct StdErrLogger {}

impl StdErrLogger {
    pub fn new() -> StdErrLogger {
        StdErrLogger {}
    }
}

impl Logger for StdErrLogger {
    fn create_dir<S>(&mut self, dir: S)
    where
        S: AsRef<str>,
    {
        eprintln!("creating directory {}", dir.as_ref());
    }

    fn run_command<CS, I, S>(&mut self, command: CS, args: I)
    where
        CS: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut raw_output = String::new();

        raw_output.push_str(command.as_ref().to_str().unwrap());

        for arg in args {
            raw_output.push_str(arg.as_ref().to_str().unwrap());
            raw_output.push(' ');
        }

        let output = Color::Green.paint(raw_output);
        eprintln!("{}", output);
    }

    fn command_output<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        let output = Style::new().dimmed().paint(s.as_ref());

        eprintln!();
        eprintln!("{}", output);
    }

    fn error<S>(&mut self, error: S)
    where
        S: AsRef<str>,
    {
        let raw_output = format!("==> {}", error.as_ref());
        let output = Color::Red.paint(raw_output);
        eprintln!("{}", output);
    }

    fn message<S>(&mut self, message: S)
    where
        S: AsRef<str>,
    {
        eprintln!("==> {}", message.as_ref());
    }
}
