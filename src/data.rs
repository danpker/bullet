extern crate serde;
extern crate serde_json;

use std::fmt;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Data {
    entries: Vec<Entry>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    text: String,
}

impl Data {
    pub fn new(&mut self, text: &str) {
        self.entries.push(Entry { text: String::from(text) });
    }

    pub fn complete(&mut self, text: &str) {
        let index: usize = text.parse().unwrap();
        self.entries.remove(index);
    }

    pub fn print(&self) {
        for (index, entry) in self.entries.iter().enumerate() {
            println!("{}: {}", index, entry);
        }
    }

    pub fn save(&self) {
        let file = File::create("list.json").unwrap();
        serde_json::to_writer(file, &self).unwrap();
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "- {}", self.text)
    }
}

pub fn load_list() -> Data {
    let file = File::open("list.json").unwrap();
    let u = serde_json::from_reader(file);

    let data = match u {
        Ok(u) => u,
        Err(_) => Data{ entries: Vec::new() },
    };
    return data;
}
