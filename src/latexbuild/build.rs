use super::Project;
use std::fs::create_dir;
use std::io;
use std::process::Command;

impl Project {
    pub fn build(&self) -> io::Result<()> {
        if !self.bin.exists() {
            println!("create bin directory {}", self.bin.to_str().unwrap());
            create_dir(&self.bin)?;
        }

        let output_dir_arg = format!("-output-directory={}", self.bin.to_str().unwrap());

        let command_output = Command::new("echo")
            .args(&["pdflatex", &output_dir_arg, self.entry.to_str().unwrap()])
            .output()?;

        let command_output_str = String::from_utf8(command_output.stdout).unwrap();

        println!("output:\n{}", command_output_str);

        return Ok(());
    }
}
