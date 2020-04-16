use std::path::PathBuf;

pub struct Includes {
    pub includes: Vec<PathBuf>,
    pub has_traversed: bool
}

impl Includes {
    pub fn new(includes: Vec<PathBuf>) -> Includes {
        Includes { includes, has_traversed: false, }
    }

    fn traverse(&mut self) {
        let mut stack: Vec<PathBuf> = Vec::new();

        for include in &self.includes {
            stack.push(PathBuf::from(include));
        }

        while !stack.is_empty() {
            // do something
        }
    }

    pub fn needs_build(&mut self) -> bool {
        if !self.has_traversed {
            self.traverse();
        }

        return true;
    }
}