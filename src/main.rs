#[macro_use] extern crate text_io;
#[macro_use] extern crate prettytable;
extern crate csv;
extern crate clap;

mod record; // Import record.rs
mod parser; // Import parser.rs
mod trie;   // Import trie.rs

// Import used libraries
use clap::{App, Arg};
use csv::ReaderBuilder;
use std::{thread, error, io::{self, Write}, str, process, time::Instant, fs::{self, OpenOptions}};
use prettytable::{Table, format};

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
                        .long("entry"))
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
                    .arg(Arg::with_name("insert")
                       .short("i")
                       .multiple(true)
                       .help("Asks to insert a new entry in the database"))
                    .arg(Arg::with_name("trie")
                       .short("t")
                       .multiple(true)
                       .help("Asks to remake the trie, based in the DATABASE.BIN file"))
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

        if let Err(err) = reparse_tries() {
            println!("Error reparsing the tries: {}", err);
            process::exit(1);
        }

    }

    // Check if it was asked to remake the trie
    if matches.occurrences_of("trie") > 0 {

        println!("=============== REPARSING THE TRIES - PLEASE WAIT!! ===============");
        if let Err(err) = reparse_tries() {
            println!("Error reparsing the tries: {}", err);
            process::exit(1);
        }
    }

    let mut name_memory_trie : trie::Trie = if let Ok(new_trie) = trie::Trie::new_from_file("name_memory_trie.bin".to_string()) {
                                                new_trie
                                            } else {
                                                trie::Trie::new()
                                            };
    let mut role_memory_trie : trie::Trie = if let Ok(new_trie) = trie::Trie::new_from_file("role_memory_trie.bin".to_string()) {
                                                new_trie
                                            } else {
                                                trie::Trie::new()
                                            };
    let mut agency_memory_trie : trie::Trie = if let Ok(new_trie) = trie::Trie::new_from_file("agency_memory_trie.bin".to_string()) {
                                                  new_trie
                                              } else {
                                                  trie::Trie::new()
                                              };

    // INSERÇÃO DE NOVA ENTRADA
    if matches.occurrences_of("insert") > 0 {
        println!("========= INSERÇÃO DE NOVO USUÁRIO =========\n");

        let records_len = fs::metadata(parser::DATABASE_FILE).unwrap().len() / record::DATA_ENTRY_SIZE as u64;
        let mut output_file = OpenOptions::new().append(true).open(parser::DATABASE_FILE).unwrap();
        let mut new_record = record::Record::new_from_stdin();
        new_record.resize();

        name_memory_trie.add(str::from_utf8(&new_record.nome).unwrap().trim_matches(char::from(0)).to_string(), records_len as u32 + 1);
        role_memory_trie.add(str::from_utf8(&new_record.descricao_cargo).unwrap().trim_matches(char::from(0)).to_string(), records_len as u32 + 1);
        agency_memory_trie.add(str::from_utf8(&new_record.orgao_exercicio).unwrap().trim_matches(char::from(0)).to_string(), records_len as u32 + 1);

        println!("{:#?}", name_memory_trie);
        if let Err(err) = name_memory_trie.save_to_file("name_memory_trie.bin") {
            println!("Erro ao salvar name-indexed trie na memória! -> {}", err);
            process::exit(1);
        }
        if let Err(err) = role_memory_trie.save_to_file("role_memory_trie.bin") {
            println!("Erro ao salvar role-indexed trie na memória! -> {}", err);
            process::exit(1);
        }
        if let Err(err) = agency_memory_trie.save_to_file("agency_memory_trie.bin") {
            println!("Erro ao salvar agency-indexed trie na memória! -> {}", err);
            process::exit(1);
        }

        output_file.write(&new_record.as_u8_array()).unwrap();

        println!("========= INSERÇÃO DE NOVO USUÁRIO FINALIZADA =========\n");

    }


    // PESQUISA DE VALORES NO ARQUIVO
    let mut entries : Vec<u32> = Vec::new();
    let mut searched = false;
    let mut threads = vec![];

    if let Some(person) = matches.value_of("person_name") {
        threads.push(thread::spawn(move || {
            let mut thread_entries : Vec<u32> = Vec::new();
            if let Some(mut entry_positions) = trie::Trie::at_from_file(&person, "name_trie.bin").unwrap() {
                thread_entries.append(&mut entry_positions);
            }
            if let Some(mut entry_positions) = trie::Trie::at_from_file(&person, "name_memory_trie.bin").unwrap() {
                thread_entries.append(&mut entry_positions);
            }

            return thread_entries;
        }));
        searched = true;
    }

    // if let Some(role) = matches.value_of("role_name").clone() {
    //     threads.push(thread::spawn(move || {
    //         let mut thread_entries : Vec<u32> = Vec::new();
    //         if let Some(mut entry_positions) = trie::Trie::at_from_file(&role, "role_trie.bin").unwrap() {
    //             thread_entries.append(&mut entry_positions);
    //         }
    //         if let Some(mut entry_positions) = trie::Trie::at_from_file(&role, "role_memory_trie.bin").unwrap() {
    //             thread_entries.append(&mut entry_positions);
    //         }
    //
    //         return thread_entries;
    //     }));
    //     searched = true;
    // }
    //
    // if let Some(agency) = matches.value_of("agency_name").clone() {
    //     threads.push(thread::spawn(move || {
    //         let mut thread_entries : Vec<u32> = Vec::new();
    //         if let Some(mut entry_positions) = trie::Trie::at_from_file(&agency, "agency_trie.bin").unwrap() {
    //             thread_entries.append(&mut entry_positions);
    //         }
    //         if let Some(mut entry_positions) = trie::Trie::at_from_file(&agency, "agency_memory_trie.bin").unwrap() {
    //             thread_entries.append(&mut entry_positions);
    //         }
    //
    //         return thread_entries;
    //     }));
    //     searched = true;
    // }

    for thread in threads {
        if let Ok(mut thread_entries) = thread.join() {
            entries.append(&mut thread_entries);
        }
    }

    if entries.len() > 0 {
       let mut csv_string : String = String::new();

       let before : Instant = Instant::now();
       let records = parser::records_from_entries(entries).unwrap();
       for mut record in records{
            csv_string += &record.generate_csv_string()
       }
       println!("Time elapsed: {:?}", Instant::now().duration_since(before));

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
    } else if searched {
        println!("No search match the filters!");
    }

}

fn reparse_tries() -> Result<(), Box<error::Error>>{
    let mut threads = vec![];

    // NAME-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating name-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("name_trie.bin".to_string(), 0) {
            println!("Error trying to generate the name-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed for name-indexed trie: {:?}", Instant::now().duration_since(before));
    }));

    // ROLE-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating role-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("role_trie.bin".to_string(), 3) {
            println!("Error trying to generate the agency-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed for role-indexed trie: {:?}", Instant::now().duration_since(before));
    }));

    // AGENCY-INDEXED TRIE
    threads.push(thread::spawn(move || {
        println!("Generating agency-indexed trie!");
        let before = Instant::now();
        if let Err(err) = trie::Trie::new_from_database("agency_trie.bin".to_string(), 4) {
            println!("Error trying to generate the agency-indexed trie: {}", err);
            process::exit(1);
        }
        println!("Time elapsed for agency-indexed trie: {:?}", Instant::now().duration_since(before));
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

    Ok(())
}
