#[macro_use] extern crate prettytable;
extern crate csv;
extern crate clap;

mod record; // Import record.rs
mod parser; // Import parser.rs
mod trie;   // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use csv::ReaderBuilder;
use std::{io::{self, Write}, process, time::Instant};
use prettytable::{Table, format};

const DEFAULT_ENTRY_POSITION : &str = "459059";
const DEFAULT_SEARCHED_NAME : &str = "MICHEL MIGUEL ELIAS TEMER LULIA";

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
                    .arg(Arg::with_name("person_name")
                        .help("Chooses the person name which will be searched in the database")
                        .takes_value(true)
                        .short("p")
                        .long("person_name")
                        .default_value(DEFAULT_SEARCHED_NAME))
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


    let name : String = matches.value_of("person_name").unwrap().parse().unwrap();
    let before : Instant = Instant::now();
    if let Some(entry_position) = trie::Trie::at_from_file(&name, "trie.bin").unwrap() {
       let mut csv_string : String = String::new();

       for entry in entry_position.iter() {
           print!("We are searching for the {}-th entry in the database --- ", entry);
           io::stdout().flush().unwrap();
           match parser::print_record_from_entry(entry - 1) {
               Some(mut record) => csv_string += &record.generate_csv_string(),
               _            => println!("Worker Not found")
           }
           println!("Time elapsed: {:?}", Instant::now().duration_since(before));
       }

       // Create the table
       let mut table = Table::from_csv(&mut ReaderBuilder::new()
                            .has_headers(false)
                            .delimiter(b';')
                            .from_reader(csv_string.as_bytes()));
       table.set_titles(row!["Nome", "Cargo", "Orgao", "Salário Bruto",
                            "13°", "IRRF", "PSS", "Demais Deducoes",
                            "Salário Líquido","Indenizações"]);
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
        table.printstd();
    } else {
        println!("Record not found!");
    }


}
