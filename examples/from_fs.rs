use std::fs::File;
use std::process::exit;

use whatsapp_export_parser::chat::Chat;

fn main() {
    if let Some(path) = std::env::args().skip(1).next() {
        let file = File::open(path).expect("Couldn't open the given file path as file");

        let chat = Chat::parse_from_reader(file).expect("Error while parsing input as chat");

        println!("{:?}", chat);
    } else {
        eprintln!("Please specify a path.");
        exit(1);
    }
}
