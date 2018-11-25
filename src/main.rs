#[macro_use]
extern crate text_io;
#[macro_use]
extern crate prettytable;
extern crate clap;
extern crate csv;

mod cli; // Import cli.rs
mod parser; // Import parser.rs
mod record; // Import record.rs
mod trie;   // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Federal Worker Blamer")
        .version("1.0.0")
        .author("Rafael B. Audibert <rbaudibert@inf.ufrgs.br>")
        .about("Search some data about the brazilian federal workers")
        .arg(
            Arg::with_name("csv")
                .help("Sets the CSV files which will be used to generate the database")
                .takes_value(true)
                .number_of_values(2)
                .short("c")
                .long("csv"),
        ).arg(
            Arg::with_name("entry")
                .help("Chooses the entry which will be searched in the database")
                .takes_value(true)
                .short("e")
                .long("entry"),
        ).arg(
            Arg::with_name("person_name")
                .help("Chooses the person name which will be searched in the database")
                .takes_value(true)
                .short("p")
                .long("person_name"),
        ).arg(
            Arg::with_name("role_name")
                .help("Chooses the role which will be searched in the database")
                .takes_value(true)
                .short("r")
                .long("role_name"),
        ).arg(
            Arg::with_name("agency_name")
                .help("Chooses the agency name which will be searched in the database")
                .takes_value(true)
                .short("a")
                .long("agency_name"),
        ).arg(
            Arg::with_name("new")
                .short("n")
                .multiple(true)
                .help("Asks to insert a new entry in the database"),
        ).arg(
            Arg::with_name("trie")
                .short("t")
                .multiple(true)
                .help("Asks to remake the trie, based in the DATABASE.BIN file"),
        ).arg(
            Arg::with_name("interactive")
                .short("i")
                .help("Runs the program in the interactive mode"),
        ).arg(
            Arg::with_name("prefix_search")
                .short("s")
                .help("Runs the program using the prefix_search (CAREFUL)"),
        ).get_matches();

    let prefix_search: bool = matches.occurrences_of("prefix_search") > 0;

    // Check if we should go to the interactive mode
    if matches.occurrences_of("interactive") > 0 {
        if let Err(err) = cli::interactive_mode(prefix_search) {
            println!("We got an error during the interactive mode: {}", err);
            process::exit(1);
        }
        process::exit(0);
    }

    // Parse CSV and build tries
    if let Some(csv_files) = matches.values_of("csv") {
        if let Err(err) = cli::parse_csv_files(csv_files) {
            println!("Error parsing the CSV {}", err);
            process::exit(1);
        }

        if let Err(err) = cli::reparse_tries() {
            println!("Error reparsing the tries: {}", err);
            process::exit(1);
        }
    }

    // Reparse the tries
    if matches.occurrences_of("trie") > 0 {
        if let Err(err) = cli::reparse_tries() {
            println!("Error reparsing the tries: {}", err);
            process::exit(1);
        }
    }

    // Instantiate the in-memory tries
    let mut name_memory_trie: trie::Trie =
        if let Ok(new_trie) = trie::Trie::new_from_file("name_memory_trie.bin".to_string()) {
            new_trie
        } else {
            trie::Trie::new()
        };
    let mut role_memory_trie: trie::Trie =
        if let Ok(new_trie) = trie::Trie::new_from_file("role_memory_trie.bin".to_string()) {
            new_trie
        } else {
            trie::Trie::new()
        };
    let mut agency_memory_trie: trie::Trie =
        if let Ok(new_trie) = trie::Trie::new_from_file("agency_memory_trie.bin".to_string()) {
            new_trie
        } else {
            trie::Trie::new()
        };

    // Create a new record in the database
    if matches.occurrences_of("new") > 0 {
        if let Err(err) = cli::create_new_entry(
            &mut name_memory_trie,
            &mut role_memory_trie,
            &mut agency_memory_trie,
        ) {
            println!("Error creating a new entry in the database: {}", err);
            process::exit(1);
        }
    }

    // Search values in the database
    if let Err(err) = cli::search_on_database(matches, prefix_search) {
        println!("Error creating a new entry in the database: {}", err);
        process::exit(1);
    }
}
