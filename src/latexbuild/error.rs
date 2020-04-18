use std::path::PathBuf;
use std::fmt::{Display, Formatter, self};

pub enum Error {
    NotFound(PathBuf),
    Encoding,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), self::fmt::Error> {
        match self {
            Error::NotFound(path_buf) => {
                let s = path_buf.to_str();

                match s {
                    Some(s) => {
                        write!(f, "{} not found", s)?;
                    },
                    None => {
                        write!(f, "? not found")?;
                    }
                }
            },
            Error::Encoding => {
                write!(f, "encoding error")?;
            }
        }

        return Ok(());
    }
}
