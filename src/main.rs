extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate chrono;

use clap::{Arg, App};
use chrono::prelude::*;

mod data;

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
            .index(2)
            .multiple(true))
        .get_matches();

    let mut data = data::load_list();

    if matches.is_present("TEXT") {
        let text_list: Vec<_> = matches.values_of("TEXT").unwrap().collect();
        let text = text_list.join(" ");

        if matches.is_present("COMMAND") {
            match matches.value_of("COMMAND").unwrap() {
                "n" => data.new(text),
                "c" => data.complete(text),
                _ => (),
            }
        }
    }

    data.save();
    let local: DateTime<Local> = Local::now();
    println!("Bullet TODO {}", local.format("%Y-%m-%d").to_string());
    data.print();
}


