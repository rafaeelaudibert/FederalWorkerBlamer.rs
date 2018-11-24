#[macro_use]
extern crate text_io;
#[macro_use]
extern crate prettytable;
extern crate clap;
extern crate csv;

mod parser; // Import parser.rs
mod record; // Import record.rs
mod trie; // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use csv::ReaderBuilder;
use prettytable::{format, Table};
use std::{
    error,
    fs::{self, OpenOptions},
    io::{self, Write},
    process, str, thread,
    time::Instant,
};

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
                .short("p")
                .help("Runs the program using the prefix_search (CAREFUL)"),
        ).get_matches();

    let prefix_search : bool = matches.occurrences_of("prefix_search") > 0; // Can safely unwrap, 'cause it has a default value

    // Check if we should go to the interactive mode
    if matches.occurrences_of("interactive") > 0 {
        if let Err(err) = interactive_mode(prefix_search) {
            println!("We got an error during the interactive mode: {}", err);
            process::exit(1);
        }
        process::exit(0);
    }

    // Parse CSV and build tries
    if let Some(csv_files) = matches.values_of("csv") {
        if let Err(err) = parse_csv_files(csv_files) {
            println!("Error parsing the CSV {}", err);
            process::exit(1);
        }

        if let Err(err) = reparse_tries() {
            println!("Error reparsing the tries: {}", err);
            process::exit(1);
        }
    }

    // Reparse the tries
    if matches.occurrences_of("trie") > 0 {
        if let Err(err) = reparse_tries() {
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
        if let Err(err) = create_new_entry(
            &mut name_memory_trie,
            &mut role_memory_trie,
            &mut agency_memory_trie,
        ) {
            println!("Error creating a new entry in the database: {}", err);
            process::exit(1);
        }
    }

    // Search values in the database
    if let Err(err) = search_on_database(matches) {
        println!("Error creating a new entry in the database: {}", err);
        process::exit(1);
    }
}

fn clear_screen(wait : bool) {
    if wait {
        print!("Press a key to continue...");
        io::stdout().flush().unwrap();
        let _ : String = if cfg!(windows) {
            read!("{}\r\n")
        } else {
            read!("{}\n")
        };
    };

    print!("{}[2J", 27 as char);
}

fn interactive_mode(prefix_search : bool) -> Result<(), Box<error::Error>> {
    clear_screen(false);

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

    loop {
        println!("======== FEDERAL WORKER BLAMER ========");
        println!("Choose an option below: ");
        println!("1. Generate the database (ALL YOUR DATA WILL BE LOST)");
        println!("2. Choose a person name to be searched in the database");
        println!("3. Choose a role in the brazilian civil service to be searched in the database");
        println!("4. Choose a brazilian federal agency to be searched in the database");
        println!("5. Insert a new worker in the database");
        println!("6. Rebuilds the database-indexes (CAREFUL, IT WILL TAKE A WHILE)");
        println!("7. Exit");

        print!("Your choice: ");
        io::stdout().flush().unwrap();
        let input: String = if cfg!(windows) {
            read!("{}\r\n")
        } else {
            read!("{}\n")
        };

        match input.as_bytes()[0] - 0x30 {
            1 => {
                println!("\nYou must pass TWO CSV files to this. The Remuneracao one, and the Cadastro one.");
                print!("Remuneracao file: ");
                io::stdout().flush().unwrap();
                let remuneracao_file: String = if cfg!(windows) {
                    read!("{}\r\n")
                } else {
                    read!("{}\n")
                };

                print!("Cadastro file: ");
                io::stdout().flush().unwrap();
                let cadastro_file: String = if cfg!(windows) {
                    read!("{}\r\n")
                } else {
                    read!("{}\n")
                };
                print!("\nThe CSV files passed in are being parsed to generate the database file.");
                io::stdout().flush().unwrap();

                let before: Instant = Instant::now();
                parser::generate_database_files(&remuneracao_file, &cadastro_file).unwrap();
                println!(
                    "\nTime elapsed: {:?}",
                    Instant::now().duration_since(before)
                );

                reparse_tries().unwrap();
                clear_screen(true);
            }
            2 => {
                print!("\nName of the to-be-searched person: ");
                io::stdout().flush().unwrap();
                let query: String = if cfg!(windows) {
                    read!("{}\r\n")
                } else {
                    read!("{}\n")
                };
                display_entries(search_person(query));
                clear_screen(true);
            }
            3 => {
                print!("\nName of the to-be-searched role: ");
                io::stdout().flush().unwrap();
                let query: String = if cfg!(windows) {
                    read!("{}\r\n")
                } else {
                    read!("{}\n")
                };
                display_entries(search_role(query));
                clear_screen(true);
            }
            4 => {
                print!("\nName of the to-be-searched agency: ");
                io::stdout().flush().unwrap();
                let query: String = if cfg!(windows) {
                    read!("{}\r\n")
                } else {
                    read!("{}\n")
                };
                display_entries(search_agency(query));
                clear_screen(true);
            }
            5 => {
                create_new_entry(
                    &mut name_memory_trie,
                    &mut role_memory_trie,
                    &mut agency_memory_trie,
                ).unwrap();
                clear_screen(true);
            }
            6 => {
                reparse_tries().unwrap();
                clear_screen(true);
            }
            7 => {
                println!("Bye bye! It was nice to have you here!! :(");
                break;
            },
            _ => {
                println!("\nINVALID CHOICE!!");
                clear_screen(true);
            },
        }
    }

    Ok(())
}

fn search_on_database(matches: clap::ArgMatches) -> Result<(), Box<error::Error>> {
    let mut entries: Vec<u32> = Vec::new();

    if let Some(person) = matches.value_of("person_name") {
        entries.append(&mut search_person(person.to_string()));
    }

    if let Some(role) = matches.value_of("role_name") {
        entries.append(&mut search_role(role.to_string()));
    }

    if let Some(agency) = matches.value_of("agency_name") {
        entries.append(&mut search_agency(agency.to_string()));
    }

    display_entries(entries);

    Ok(())
}

fn display_entries(entries: Vec<u32>) {
    if entries.len() > 0 {
        let mut csv_string: String = String::new();

        let before: Instant = Instant::now();
        let records = parser::records_from_entries(entries).unwrap();
        for mut record in records {
            csv_string += &record.generate_csv_string()
        }
        println!("\nTime elapsed: {:?}", Instant::now().duration_since(before));

        // Create the table
        let mut table = Table::from_csv(
            &mut ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b';')
                .from_reader(csv_string.as_bytes()),
        );
        table.set_titles(row![
            "Nome",
            "Cargo",
            "Orgao",
            "Salário Bruto",
            "13°",
            "IRRF",
            "PSS",
            "Demais Deducoes",
            "Salário Líquido",
            "Indenizações"
        ]);
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
        table.printstd();
    } else {
        println!("No search match the filters!");
    }
}

fn search_person(person: String) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    if let Some(mut entry_positions) = trie::Trie::at_from_file(&person, "name_trie.bin").unwrap() {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("name_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&person, "name_memory_trie.bin").unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }

    return entries;
}

fn search_role(role: String) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    if let Some(mut entry_positions) = trie::Trie::at_from_file(&role, "role_trie.bin").unwrap() {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("role_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&role, "role_memory_trie.bin").unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }

    return entries;
}

fn search_agency(agency: String) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    if let Some(mut entry_positions) = trie::Trie::at_from_file(&agency, "agency_trie.bin").unwrap()
    {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("agency_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&agency, "agency_memory_trie.bin").unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }

    return entries;
}

fn create_new_entry(
    name_trie: &mut trie::Trie,
    role_trie: &mut trie::Trie,
    agency_trie: &mut trie::Trie,
) -> Result<(), Box<error::Error>> {
    println!("========= INSERÇÃO DE NOVO USUÁRIO =========\n");

    let records_len =
        fs::metadata(parser::DATABASE_FILE).unwrap().len() / record::DATA_ENTRY_SIZE as u64;
    let mut output_file = OpenOptions::new()
        .append(true)
        .open(parser::DATABASE_FILE)
        .unwrap();
    let mut new_record = record::Record::new_from_stdin();
    new_record.resize();

    name_trie.add(
        str::from_utf8(&new_record.nome)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string(),
        records_len as u32 + 1,
    );
    role_trie.add(
        str::from_utf8(&new_record.descricao_cargo)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string(),
        records_len as u32 + 1,
    );
    agency_trie.add(
        str::from_utf8(&new_record.orgao_exercicio)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string(),
        records_len as u32 + 1,
    );

    name_trie.save_to_file("name_memory_trie.bin").unwrap();
    role_trie.save_to_file("role_memory_trie.bin").unwrap();
    agency_trie.save_to_file("agency_memory_trie.bin").unwrap();

    output_file.write(&new_record.as_u8_array()).unwrap();

    println!("========= INSERÇÃO DE NOVO USUÁRIO FINALIZADA =========\n");

    Ok(())
}

fn parse_csv_files(mut csv_files: clap::Values) -> Result<(), Box<error::Error>> {
    print!("The CSV files passed in are being parsed to generate the database file.");
    io::stdout().flush().unwrap();

    let before: Instant = Instant::now();
    parser::generate_database_files(csv_files.next().unwrap(), csv_files.next().unwrap()).unwrap();
    println!(
        "\nTime elapsed: {:?}",
        Instant::now().duration_since(before)
    );

    Ok(())
}
fn reparse_tries() -> Result<(), Box<error::Error>> {
    let mut threads = vec![];

    println!("=============== REPARSING THE TRIES - PLEASE WAIT!! ===============");

    // NAME-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating name-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("name_trie.bin".to_string(), 0) {
            println!("Error trying to generate the name-indexed trie: {}", err);
            process::exit(1);
        }
        println!(
            "\nTime elapsed for name-indexed trie: {:?}",
            Instant::now().duration_since(before)
        );
    }));

    // ROLE-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating role-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("role_trie.bin".to_string(), 3) {
            println!("Error trying to generate the agency-indexed trie: {}", err);
            process::exit(1);
        }
        println!(
            "\nTime elapsed for role-indexed trie: {:?}",
            Instant::now().duration_since(before)
        );
    }));

    // AGENCY-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating agency-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("agency_trie.bin".to_string(), 4) {
            println!("Error trying to generate the agency-indexed trie: {}", err);
            process::exit(1);
        }
        println!(
            "\nTime elapsed for agency-indexed trie: {:?}",
            Instant::now().duration_since(before)
        );
    }));

    for thread in threads {
        if let Err(err) = thread.join() {
            println!("Error trying to join threads: {:?}", err);
        }
    }

    // Remove old files which hold the stuff in the memory
    if fs::metadata("name_memory_trie.bin").is_ok() {
        fs::remove_file("name_memory_trie.bin")?;
    }
    if fs::metadata("role_memory_trie.bin").is_ok() {
        fs::remove_file("role_memory_trie.bin")?;
    }
    if fs::metadata("agency_memory_trie.bin").is_ok() {
        fs::remove_file("agency_memory_trie.bin")?;
    }

    println!("=============== FINISHED!! ===============");
    Ok(())
}
