use ansi_term::{Color, Style};
use std::ffi::OsStr;

/// Logger
pub trait Logger {
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
    fn run_command<CS, I, S>(&mut self, command: CS, args: I)
    where
        CS: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut raw_output = String::new();
        let s = match command.as_ref().to_str() {
            Some(s) => s,
            _ => return,
        };

        raw_output.push_str(s);
        raw_output.push(' ');

        for arg in args {
            let s = match arg.as_ref().to_str() {
                Some(s) => s,
                _ => return,
            };

            raw_output.push_str(s);
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
