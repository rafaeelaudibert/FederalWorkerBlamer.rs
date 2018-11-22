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
                        .long("person_name"))
                    .arg(Arg::with_name("role_name")
                        .help("Chooses the role which will be searched in the database")
                        .takes_value(true)
                        .short("r")
                        .long("role_name"))
                    .arg(Arg::with_name("agency_name")
                        .help("Chooses the agency name which will be searched in the database")
                        .takes_value(true)
                        .short("a")
                        .long("agency_name"))

                    .get_matches();

    if let Some(mut csv_files) = matches.values_of("csv") {
        let mut before : Instant;

        print!("The CSV files passed in are being parsed to generate the database file.");
        io::stdout().flush().unwrap();
        before = Instant::now();
        if let Err(err) = parser::generate_database_files(csv_files.next().unwrap(), csv_files.next().unwrap()) {
           println!("Error trying to generate the database file: {}", err);
           process::exit(1);
        }
        println!("\nTime elapsed: {:?}", Instant::now().duration_since(before));

        println!("Generating name-indexed trie!");
        before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("name_trie.bin".to_string(), 0) {
            println!("Error trying to generate the name-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));

        println!("Generating role-indexed trie!");
        before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("role_trie.bin".to_string(), 3) {
            println!("Error trying to generate the role-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));

        println!("Generating agency-indexed trie!");
        before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("agency_trie.bin".to_string(), 4) {
            println!("Error trying to generate the agency-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    }


    let mut entries : Vec<u32> = Vec::new();
    let before : Instant = Instant::now();

    if let Some(person) = matches.value_of("person_name") {
        if let Some(mut entry_positions) = trie::Trie::at_from_file(&person, "name_trie.bin").unwrap() {
            entries.append(&mut entry_positions);
        }
    }

    if let Some(role) = matches.value_of("role_name") {
        if let Some(mut entry_positions) = trie::Trie::at_from_file(&role, "role_trie.bin").unwrap() {
            entries.append(&mut entry_positions);
        }
    }

    if let Some(agency) = matches.value_of("agency_name") {
        if let Some(mut entry_positions) = trie::Trie::at_from_file(&agency, "agency_trie.bin").unwrap() {
            entries.append(&mut entry_positions);
        }
    }

    if entries.len() > 0 {
       let mut csv_string : String = String::new();

       for entry in entries.iter() {
           print!("We are searching for the {}-th entry in the database --- ", entry);
           io::stdout().flush().unwrap();
           match parser::record_from_entry(entry - 1) {
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
