extern crate clap;
#[macro_use] extern crate serde_derive;

use clap::{Arg, App};

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
            .index(2))
        .get_matches();

    let mut data = data::load_list();

    if matches.is_present("TEXT") {
        let text = matches.value_of("TEXT").unwrap();

        if matches.is_present("COMMAND") {
            match matches.value_of("COMMAND").unwrap() {
                "n" => data.new(text),
                "c" => data.complete(text),
                _ => (),
            }
        }
    }

    data.save();
    data.print();
}


