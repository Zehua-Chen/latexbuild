use json;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::path::PathBuf;

pub enum Error {
    PathNotFound(PathBuf),
    IO(io::Error),
    JsonParsing(json::Error),
    WrongConfigFormat(String),
    NoEntry,
    Encoding,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), self::fmt::Error> {
        match self {
            Error::PathNotFound(path_buf) => {
                let s = path_buf.to_str();

                match s {
                    Some(s) => {
                        write!(f, "{} not found", s)?;
                    }
                    None => {
                        write!(f, "? not found")?;
                    }
                }
            }
            Error::IO(_error) => {
                write!(f, "io error")?;
            }
            Error::JsonParsing(_error) => {
                write!(f, "json syntax error")?;
            }
            Error::Encoding => {
                write!(f, "encoding error")?;
            }
            Error::WrongConfigFormat(message) => {
                write!(f, "{}", message)?;
            }
            Error::NoEntry => {
                write!(f, "no entry specified")?;
            }
        }

        return Ok(());
    }
}
