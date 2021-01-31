use chrono::{DateTime, Local};
use std::path::Path;

pub struct VecDB {
    path: String,
    vec: Vec<String>    
}

impl VecDB {
    pub fn new() -> Result<VecDB, std::io::Error> {
        let path = String::from("iv.db");
        let mut vec: Vec<String>= Vec::new();

        if Path::new(&path).exists() {
            let contents = std::fs::read_to_string(&path)?;
            for line in contents.lines() {
                vec.push(line.to_owned());
            }
        }
        Ok(VecDB { path, vec })
    }

    pub fn insert(&mut self, value: String) {
        let mut contents = String::new();
        contents.push_str(&value);
        contents.push('\t');
        contents.push_str(&Local::now().to_string());
        contents.push('\n');
        self.vec.push(contents);
    }

    pub fn get(self, key: String) {
        let default = String::from("NOT FOUND");
        let index: usize = key.trim().parse::<usize>().expect("Please type a number!");
        let mut values = self.vec.get(index).unwrap_or(&default).split('\t');
        let value = values.next().unwrap();
        let datetime = values.next().unwrap().parse::<DateTime<Local>>().expect("Unable to parse stored datetime");        
        println!("{} at time {}", value, datetime);
    }

    pub fn wipe(mut self) {
        self.vec = Vec::new();
        self.write()
    }

    pub fn dump(self) {
        let mut contents = String::new();
        for value in &self.vec {
            contents.push_str(value);
            contents.push('\n');
        }
        println!("{}", contents);
    }

    pub fn write(&mut self) {
        let mut contents = String::new();
        for value in &self.vec {
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write(&self.path, contents).unwrap();
    }
}
