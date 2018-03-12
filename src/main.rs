extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::fs::File;
use std::error::Error;
use clap::{Arg, App, SubCommand};

#[derive(Serialize, Deserialize)]
struct Data {
    entries: Vec<Entry>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    text: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "- {}", self.text)
    }
}

fn main() {
    let matches = App::new("Bullet")
        .version("1.0")
        .about("Bullet Journaling in your Terminal")
        .arg(Arg::with_name("COMMAND")
            .help("The command to run")
            .required(false)
            .index(1))
        .arg(Arg::with_name("TEXT")
            .help("The text for the command")
            .required(false)
            .index(2))
        .get_matches();

    let mut data = match load_list() {
        Ok(data) => data,
        Err(_) => Data{ entries: Vec::new() },
    };

    if matches.is_present("TEXT") {
        let text = matches.value_of("TEXT").unwrap();

        if matches.is_present("COMMAND") {
            match matches.value_of("COMMAND").unwrap() {
                "n" => new(text, &mut data),
                _ => (),
            }
        }
    } else {
    }

    write_list(&data);
    print_list(&data);
}

fn new(text: &str, data: &mut Data) {
    data.entries.push(Entry { text: String::from(text) });
    println!("Added: {}", text);
}

fn print_list(data: &Data) {
    for entry in data.entries.iter() {
        println!("{}", entry);
    }
}

fn write_list(data: &Data) -> Result<bool, Box<Error>> {
    let file = File::create("list.json")?;
    serde_json::to_writer(file, &data)?;

    Ok(true)
}

fn load_list() -> Result<Data, Box<Error>> {
    let file = File::open("list.json")?;

    let u = serde_json::from_reader(file)?;

    Ok(u)
}
