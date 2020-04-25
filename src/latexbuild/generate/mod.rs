use super::Error;
use std::fs::File;
use std::io::BufWriter;

mod make;

pub use make::*;

pub trait Generate {
    fn generate(&self, writer: &mut BufWriter<File>) -> Result<(), Error>;
}
