use chrono::{DateTime, Local};

use std::collections::HashMap;
use std::path::Path;

pub struct HashDB {
    path: String,
    map: HashMap<String, String>    
}

impl HashDB {
    pub fn new() -> Result<HashDB, std::io::Error> {
        let path = String::from("kv.db");
        let mut map = HashMap::new();

        if Path::new(&path).exists() {
            let contents = std::fs::read_to_string(&path)?;
            for line in contents.lines() {
                let mut chunks = line.splitn(2, '\t');
                let key = chunks.next().expect("No key!");
                let value = chunks.next().expect("No value!");
                map.insert(key.to_owned(), value.to_owned());
            }   
        }
        
        Ok(HashDB { path, map })
    }

    pub fn insert(&mut self, key: String, value: String) {
        let mut contents = String::new();
        contents.push_str(&value);
        contents.push('\t');
        contents.push_str(&Local::now().to_string());
        self.map.insert(key, contents);
    }

    pub fn get(self, key: String) {
        let default = String::from("NOT FOUND");
        let mut values = self.map.get(&key).unwrap_or(&default).split('\t');
        let value = values.next().unwrap();
        let datetime = values.next().unwrap().parse::<DateTime<Local>>().expect("Unable to parse stored datetime");        
        println!("{} at time {}", value, datetime);
    }

    pub fn wipe(mut self) {
        self.map = HashMap::new();
        self.write()
    }

    pub fn dump(self) {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        println!("{}", contents);
    }

    pub fn write(&mut self) {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write(&self.path, contents).unwrap();
    }
}
