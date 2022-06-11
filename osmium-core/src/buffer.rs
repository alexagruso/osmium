use std::fs::{self, File};
use std::io::{Write, ErrorKind};
use std::path::Path;

pub struct Buffer {
    pub name: Option<String>,
    pub contents: Vec<String>,
}

impl Buffer {
    pub fn new(name: Option<String>) -> Buffer {
        match name {
            Some(name) => {
                let contents: Vec<String> = match fs::read_to_string(Path::new(&name)) {
                    Ok(file) => {
                        let mut lines = Vec::new();

                        for line in file.lines() {
                            lines.push(line.to_string() + "\n");
                        }

                        lines
                    },
                    Err(why) => match why.kind() {
                        ErrorKind::NotFound => vec!["".to_string()],
                        _ => panic!("Error opening {}: {}", name, why),
                    },
                };

                Buffer {
                    name: Some(name),
                    contents,
                }
            },
            None => Buffer {
                name,
                contents: Vec::new(),
            },
        }
    }

    pub fn append(&mut self, line: String) {
        self.contents.push(line);
    }

    pub fn print(&self) {
        for line in self.contents.iter() {
            print!("{}", line);
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn write_file(&self) {
        let mut file = match self.name.clone() {
            Some(name) => File::create(Path::new(&name)).unwrap(),
            None => panic!("Attempted to write to a file with no name!"),
        };

        for line in self.contents.iter() {
            file.write(line.as_bytes()).unwrap();
        }
    }
}
