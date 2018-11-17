extern crate csv;
extern crate clap;

// Import record.rs
mod record;
use record::Record;

// Import used libraries
use csv::{ReaderBuilder, StringRecord};
use clap::{App, Arg};
use std::{error::Error,
          process,
          str,
          fs,
          fs::File,
          time::Instant,
          io::{Seek, SeekFrom, Read, Write}};

const DEFAULT_ENTRY_POSITION : u64 = 459_059;

fn generate_binary(input_file : &str) -> Result<(), Box<Error>> {
    let mut csv_salary_reader = ReaderBuilder::new().delimiter(b';').from_path(input_file)?;
    let mut csv_info_reader = ReaderBuilder::new().delimiter(b';').from_path("csv/201808_Cadastro.csv" )?;
    let mut output_file = File::create("remuneracao.bin")?;

    let salary_values = csv_salary_reader.records().map(|r| r.unwrap());
    let mut info_values = csv_info_reader.records().map(|r| r.unwrap()).peekable();
    let mut sem_informacao_vec : Vec<u8> = Vec::new();

    for elem in vec!(83, 101, 109, 32, 105, 110, 102, 111, 114, 109, 97, 239, 191, 189, 239, 191, 189, 111) {
        sem_informacao_vec.push(elem);
    }

    for salary_value in salary_values {

        let mut info_value : StringRecord = StringRecord::new();
        while let Some(possible_info_value) = Some(info_values.next().unwrap()) {
            if possible_info_value.get(0).unwrap().as_bytes().to_vec() != salary_value[2].as_bytes().to_vec() {
                continue;
            } else if possible_info_value.get(4).unwrap().as_bytes().to_vec() != sem_informacao_vec {
                info_value = possible_info_value; // Primeira entrada com aquele ID tem uma profissao
                break;
            } else if info_values.peek().unwrap().get(0).unwrap().as_bytes().to_vec() == salary_value[2].as_bytes().to_vec() {
                // Ultima posição não tinha profissão, mas próxima vai ter, então pego a proxima
                info_value = info_values.next().unwrap();
                break;
            } else {
                //Se a pessoa não tem informação no db, eu uso "Sem informacao" mesmo
                info_value = possible_info_value;
                break;
            }
        }

        let mut record = Record {
            id: salary_value[2].as_bytes().to_vec(),
            cpf: salary_value[3].as_bytes().to_vec(),
            nome: salary_value[4].as_bytes().to_vec(),
            descricao_cargo: info_value[4].as_bytes().to_vec(),
            orgao_lotacao: info_value[18].as_bytes().to_vec(),
            orgao_exercicio: info_value[24].as_bytes().to_vec(),
            remuneracao_basica_bruta_rs: salary_value[5].as_bytes().to_vec(),
            gratificacao_natalina_rs: salary_value[9].as_bytes().to_vec(),
            ferias_rs: salary_value[13].as_bytes().to_vec(),
            outras_remuneracoes_eventuais_rs: salary_value[15].as_bytes().to_vec(),
            irrf_rs: salary_value[17].as_bytes().to_vec(),
            pss_rgps_rs: salary_value[19].as_bytes().to_vec(),
            demais_deducoes_rs: salary_value[21].as_bytes().to_vec(),
            remuneracao_apos_deducoes_obrigatorias_rs: salary_value[29].as_bytes().to_vec(),
            total_verbas_indenizatorias_rs: salary_value[37].as_bytes().to_vec(),
            data_inicio_afastamento: info_value[29].as_bytes().to_vec(),
            data_termino_afastamento: info_value[30].as_bytes().to_vec(),
            regime_contratacao: info_value[31].as_bytes().to_vec(),
            jornada_trabalho: info_value[32].as_bytes().to_vec(),
            data_ingresso_cargo: info_value[33].as_bytes().to_vec(),
            data_ingresso_orgao: info_value[35].as_bytes().to_vec()
        };

        record.resize();
        output_file.write(&record.as_u8_array()).unwrap();

    }

    Ok(())
}

fn exceeds_database_size(entry_position : u64) -> bool {
    let metadata = fs::metadata("remuneracao.bin").unwrap();
    if metadata.len() > entry_position * record::DATA_ENTRY_SIZE as u64 { false } else { true }
}

fn print_record_from_entry(entry: u64) -> Option<Record> {
    print_record_from_offset(entry * record::DATA_ENTRY_SIZE as u64)
}

fn print_record_from_offset(offset : u64) -> Option<Record> {
    let mut f = File::open("remuneracao.bin").unwrap();
    let mut buffer : Vec<u8>;
    let mut record = Record::default();

    f.seek(SeekFrom::Start(offset)).unwrap();

    for (i, bytes) in record::RECORD_SIZES.iter().enumerate() {

        buffer = vec![0; *bytes as usize];
        f.read_exact(&mut buffer).unwrap();

        let text : &str = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0));

        match i {
            0 => record.nome = text.as_bytes().to_vec(),
            1 => record.id = text.as_bytes().to_vec(),
            2 => record.cpf = text.as_bytes().to_vec(),
            3 => record.descricao_cargo = text.as_bytes().to_vec(),
            4 => record.orgao_lotacao = text.as_bytes().to_vec(),
            5 => record.orgao_exercicio = text.as_bytes().to_vec(),
            6 => record.remuneracao_basica_bruta_rs = text.as_bytes().to_vec(),
            7 => record.gratificacao_natalina_rs = text.as_bytes().to_vec(),
            8 => record.ferias_rs = text.as_bytes().to_vec(),
            9 => record.outras_remuneracoes_eventuais_rs = text.as_bytes().to_vec(),
            10 => record.irrf_rs = text.as_bytes().to_vec(),
            11 => record.pss_rgps_rs = text.as_bytes().to_vec(),
            12 => record.demais_deducoes_rs = text.as_bytes().to_vec(),
            13 => record.remuneracao_apos_deducoes_obrigatorias_rs = text.as_bytes().to_vec(),
            14 => record.total_verbas_indenizatorias_rs = text.as_bytes().to_vec(),
            15 => record.data_inicio_afastamento = text.as_bytes().to_vec(),
            16 => record.data_termino_afastamento = text.as_bytes().to_vec(),
            17 => record.regime_contratacao = text.as_bytes().to_vec(),
            18 => record.jornada_trabalho = text.as_bytes().to_vec(),
            19 => record.data_ingresso_cargo = text.as_bytes().to_vec(),
            20 => record.data_ingresso_orgao = text.as_bytes().to_vec(),
            _ => println!("Error!!")
        }
    }

    Some(record)
}

fn main() {
    let mut before : Instant;

    let matches = App::new("Funcionalismo Publico Parser")
                    .version("1.0")
                    .author("Rafael B. Audibert <rbaudibert@inf.ufrgs.br>")
                    .about("Pesquise alguns dados sobre o salário dos servidores públicos federais brasileiros")
                    .arg(Arg::with_name("csv")
                        .help("Sets the CSV file which will be used to generate the information")
                        .takes_value(true)
                        .short("c")
                        .long("csv"))
                    .arg(Arg::with_name("entry")
                        .help("Chooses the entry which will be searched in the database")
                        .takes_value(true)
                        .short("e")
                        .long("entry"))
                    .get_matches();

    if let Some(csv_file) = matches.value_of("csv") {
        println!("A csv file was passed in: {}\n\
                  It will be parsed to generate the database file", csv_file);
        before = Instant::now();
        if let Err(err) = generate_binary(csv_file) {
           println!("Error trying to generate the database file: {}", err);
           process::exit(1);
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    }

    let mut entry_position : u64 = DEFAULT_ENTRY_POSITION;
    if let Some(position) = matches.value_of("entry") {
        entry_position = position.parse().unwrap()
    }

    if exceeds_database_size(entry_position) {
        println!("Error trying to acess this position in the database");
        process::exit(1);
    } else {
        before = Instant::now();
        println!("We are searching for the {}-th entry in the database", entry_position);
        match print_record_from_entry(entry_position) {
            Some(record) => println!("{}", record),
            _            => println!("Not found")
        }
        println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    }

}
