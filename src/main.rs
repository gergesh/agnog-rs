use clap::{Arg, App};
use encoding::all::ISO_8859_8;
use encoding::{Encoding, DecoderTrap};
use std::fs::File;
use std::io::{BufReader, Read};

const ENTRY_SIZE: usize = 403;

fn is_empty(c: char) -> bool {
    c == '\0' || c.is_whitespace()
}

fn main() {
    let matches = App::new("agnog-rs")
        .version("0.2.0")
        .about("A program to convert the Agron database file to a \
               UTF-8 encoded csv")
        .arg(Arg::with_name("file")
             .help("3.cvf file")
             .required(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let file = File::open(file)
        .expect("Could not open provided file.");

    let mut reader = BufReader::new(file);
    let mut buf = [0; ENTRY_SIZE];

    while reader.read_exact(&mut buf).is_ok() {
        let utf8 = ISO_8859_8.decode(&buf, DecoderTrap::Strict)
            .expect("Could not decode read string.");

        let parts = utf8.split(',');
        let parts = parts.map(|p| p.trim_matches(is_empty));
        println!("{}", parts.collect::<Vec<&str>>().join(","));
    }
}
