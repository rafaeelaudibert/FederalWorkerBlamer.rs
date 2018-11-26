use csv::ReaderBuilder;
use parser;
use prettytable::{format, Table};
use record;
use std::{
    error,
    fs::{self, OpenOptions},
    io::{self, Write},
    process, str, thread,
    time::Instant,
};
use trie;

fn clear_screen(wait: bool) {
    if wait {
        print!("Press a key to continue...");
        io::stdout().flush().unwrap();
        let _: String = if cfg!(windows) {
            read!("{}\r\n")
        } else {
            read!("{}\n")
        };
    };

    print!("{}[2J", 27 as char);
}

pub fn interactive_mode(prefix_search: bool) -> Result<(), Box<error::Error>> {
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

        if input.as_bytes().len() > 0 {
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
                        "\nTime elapsed in the CSV parsing: {:?}",
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

                    let mut entries = search_person(query, prefix_search);
                    entries.sort();
                    entries.dedup();

                    display_entries(entries);
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

                    let mut entries = search_role(query, prefix_search);
                    entries.sort();
                    entries.dedup();

                    display_entries(entries);
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

                    let mut entries = search_agency(query, prefix_search);
                    entries.sort();
                    entries.dedup();

                    display_entries(entries);
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
                }
                _ => {
                    println!("\nINVALID CHOICE!!");
                    clear_screen(true);
                }
            }
        } else {
            println!("\nINVALID CHOICE!!");
            clear_screen(true);
        }
    }

    Ok(())
}

pub fn search_on_database(matches: clap::ArgMatches, prefix_search : bool, or : bool) -> Result<(), Box<error::Error>> {
    let mut person_entries : Vec<u32> = Vec::new();
    let mut role_entries : Vec<u32> = Vec::new();
    let mut agency_entries : Vec<u32> = Vec::new();

    if let Some(person) = matches.value_of("person_name") {
        person_entries = search_person(person.to_string(), prefix_search);
    }

    if let Some(role) = matches.value_of("role_name") {
        role_entries = search_role(role.to_string(), prefix_search);
    }

    if let Some(agency) = matches.value_of("agency_name") {
        agency_entries = search_agency(agency.to_string(), prefix_search);
    }

    let mut entries: Vec<u32> = Vec::new();
    if !or { // === if and

        if matches.occurrences_of("person_name") > 0 {
            if matches.occurrences_of("role_name") > 0 {
                if matches.occurrences_of("agency_name") > 0 {
                    // Has person, role and agency
                    let mut partial_entries : Vec<u32> = Vec::new();

                    for entry in person_entries {
                        if role_entries.contains(&entry) { partial_entries.push(entry) }
                    }

                    for entry in partial_entries {
                        if agency_entries.contains(&entry) { entries.push(entry) }
                    }

                } else {
                    // Has person and role
                    //println!("PERSON_ENTRIES: {:?} - AGENCY_ENTRIES: ");
                    for entry in person_entries {
                        if role_entries.contains(&entry) { entries.push(entry) }
                    }

                }
            } else {
                if matches.occurrences_of("agency_name") > 0 {
                    // Has person and agency
                    for entry in person_entries {
                        if agency_entries.contains(&entry) { entries.push(entry) }
                    }

                } else {
                    // Has person
                    entries = person_entries;

                }
            }
        } else if matches.occurrences_of("role_name") > 0 {
            if matches.occurrences_of("agency_name") > 0 {
                // Has role and agency
                for entry in role_entries {
                    if agency_entries.contains(&entry) { entries.push(entry) }
                }

            } else {
                // Has role
                entries = role_entries;

            }
        } else {
            // Has agency
            entries = agency_entries;

        }

        display_entries(entries);
        return Ok(())
    }

    entries.append(&mut person_entries);
    entries.append(&mut role_entries);
    entries.append(&mut agency_entries);
    entries.sort();
    entries.dedup();
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
        println!(
            "Time elapsed to parse the records from the file: {:?}",
            Instant::now().duration_since(before)
        );

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

fn search_person(person: String, prefix_search : bool) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    let before = Instant::now();
    if let Some(mut entry_positions) = trie::Trie::at_from_file(&person, "name_trie.bin", prefix_search).unwrap() {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("name_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&person, "name_memory_trie.bin", prefix_search).unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }
    println!("\nTime elapsed to search the name trie: {:?}", Instant::now().duration_since(before));

    return entries;
}

fn search_role(role: String, prefix_search : bool) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    let before = Instant::now();
    if let Some(mut entry_positions) = trie::Trie::at_from_file(&role, "role_trie.bin", prefix_search).unwrap() {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("role_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&role, "role_memory_trie.bin", prefix_search).unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }
    println!("\nTime elapsed to search the role trie: {:?}", Instant::now().duration_since(before));

    return entries;
}

fn search_agency(agency: String, prefix_search : bool) -> Vec<u32> {
    let mut entries: Vec<u32> = Vec::new();

    let before = Instant::now();
    if let Some(mut entry_positions) = trie::Trie::at_from_file(&agency, "agency_trie.bin", prefix_search).unwrap()
    {
        entries.append(&mut entry_positions);
    }
    if fs::metadata("agency_memory_trie.bin").is_ok() {
        if let Some(mut entry_positions) =
            trie::Trie::at_from_file(&agency, "agency_memory_trie.bin", prefix_search).unwrap()
        {
            entries.append(&mut entry_positions);
        }
    }
    println!("\nTime elapsed to search the agency trie: {:?}", Instant::now().duration_since(before));

    return entries;
}

pub fn create_new_entry(
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

    // Name-indexed trie insertion
    let name = str::from_utf8(&new_record.nome)
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    let name_split : Vec<&str> = name.split_whitespace().collect();
    for i in 0..name_split.len() + 1 {
        for j in i+1..name_split.len() + 1 {
            println!("{:?}", name_split[i..j].join(" "));
            name_trie.add(name_split[i..j].join(" "), records_len as u32 + 1); // Add each of the words
        }
    }

    // Role-indexed trie insertion
    let role = str::from_utf8(&new_record.descricao_cargo)
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    let role_split : Vec<&str> = role.split_whitespace().collect();
    for i in 0..role_split.len() + 1 {
        for j in i+1..role_split.len() + 1 {
            role_trie.add(role_split[i..j].join(" "), records_len as u32 + 1); // Add each of the words
        }
    }


    // Agency-indexed trie insertion
    let agency = str::from_utf8(&new_record.orgao_exercicio)
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    let agency_split : Vec<&str> = agency.split_whitespace().collect();
    for i in 0..agency_split.len() + 1 {
        for j in i+1..agency_split.len() + 1 {
            agency_trie.add(agency_split[i..j].join(" "), records_len as u32 + 1); // Add each of the words
        }
    }

    name_trie.save_to_file("name_memory_trie.bin").unwrap();
    role_trie.save_to_file("role_memory_trie.bin").unwrap();
    agency_trie.save_to_file("agency_memory_trie.bin").unwrap();

    output_file.write(&new_record.as_u8_array()).unwrap();

    println!("========= INSERÇÃO DE NOVO USUÁRIO FINALIZADA =========\n");

    Ok(())
}

pub fn parse_csv_files(mut csv_files: clap::Values) -> Result<(), Box<error::Error>> {
    print!("The CSV files passed in are being parsed to generate the database file.");
    io::stdout().flush().unwrap();

    let before: Instant = Instant::now();
    parser::generate_database_files(csv_files.next().unwrap(), csv_files.next().unwrap()).unwrap();
    println!(
        "\nTime elapsed in the CSV parsing: {:?}",
        Instant::now().duration_since(before)
    );

    Ok(())
}

pub fn reparse_tries() -> Result<(), Box<error::Error>> {
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
