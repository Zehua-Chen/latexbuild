use std::fs::File;
use std::io::{BufWriter, Result};

mod make;

pub use make::*;

pub trait Generate {
    fn generate(&self, writer: &mut BufWriter<File>) -> Result<()>;
}
