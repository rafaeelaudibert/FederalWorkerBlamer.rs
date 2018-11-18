extern crate csv;
extern crate clap;


mod record; // Import record.rs
mod parser; // Import parser.rs
mod trie;   // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use std::{process, time::Instant};

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
        println!("The CSV files passed in will be parsed to generate the database file");
        let before : Instant = Instant::now();
        if let Err(err) = parser::generate_database_files(csv_files.next().unwrap(), csv_files.next().unwrap()) {
           println!("Error trying to generate the database file: {}", err);
           process::exit(1);
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    }

    let entry_position : u64 = matches.value_of("entry").unwrap().parse().unwrap();
    let before : Instant = Instant::now();
    println!("We are searching for the {}-th entry in the database", entry_position);
    match parser::print_record_from_entry(entry_position - 1) {
        Some(record) => println!("{}", record),
        _            => println!("Worker Not found")
    }
    println!("Time elapsed: {:?}", Instant::now().duration_since(before));

}
