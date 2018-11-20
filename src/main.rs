extern crate csv;
extern crate clap;


mod record; // Import record.rs
mod parser; // Import parser.rs
mod trie;   // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use std::{io::{self, Write}, process, time::Instant};

const DEFAULT_ENTRY_POSITION : &str = "459059";

fn main() {

    let matches = App::new("Funcionalismo Publico Parser")
                    .version("1.0")
                    .author("Rafael B. Audibert <rbaudibert@inf.ufrgs.br>")
                    .about("Pesquise alguns dados sobre o salário dos servidores públicos federais brasileiros")
                    .arg(Arg::with_name("csv")
                        .help("Sets the CSV files which will be used to generate the information")
                        .takes_value(true)
                        .number_of_values(2)
                        .short("c")
                        .long("csv"))
                    .arg(Arg::with_name("entry")
                        .help("Chooses the entry which will be searched in the database")
                        .takes_value(true)
                        .short("e")
                        .long("entry")
                        .default_value(DEFAULT_ENTRY_POSITION))
                    .get_matches();

    if let Some(mut csv_files) = matches.values_of("csv") {
        print!("The CSV files passed in are being parsed to generate the database file.");
        io::stdout().flush().unwrap();
        let before : Instant = Instant::now();
        if let Err(err) = parser::generate_database_files(csv_files.next().unwrap(), csv_files.next().unwrap()) {
           println!("Error trying to generate the database file: {}", err);
           process::exit(1);
        }
        println!("\nTime elapsed: {:?}", Instant::now().duration_since(before));
    }

    //let entry_position : u64 = matches.value_of("entry").unwrap().parse().unwrap();
    let before : Instant = Instant::now();
    if let Some(entry_position) = trie::Trie::at_from_file("MICHEL MIGUEL ELIAS TEMER LULIA", "trie.bin").unwrap() {
        let entry = entry_position[0];
        println!("We are searching for the {}-th entry in the database", entry);
        match parser::print_record_from_entry(entry - 1) {
            Some(record) => println!("{}", record),
            _            => println!("Worker Not found")
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    } else {
        println!("Record not found!");
    }
}
