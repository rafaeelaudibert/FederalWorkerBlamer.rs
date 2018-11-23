use record;
use record::{Record};
use csv::{ReaderBuilder, StringRecord};
use std::{error::Error,
          str,
          fs,
          fs::File,
          io::{self, Seek, SeekFrom, Read, Write}
         };


pub const DATABASE_FILE : &str = "database.bin";

pub fn generate_database_files(salary_file : &str, info_file: &str) -> Result<(), Box<Error>> {
    let mut csv_salary_reader = ReaderBuilder::new().delimiter(b',').has_headers(false).from_path(salary_file)?;
    let mut csv_info_reader = ReaderBuilder::new().delimiter(b';').from_path(info_file)?;
    let mut output_file = File::create(DATABASE_FILE)?;
    let mut counter : u32 = 0;


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
            jornada_trabalho: info_value[32].as_bytes().to_vec(),
            data_ingresso_cargo: info_value[33].as_bytes().to_vec(),
            data_ingresso_orgao: info_value[35].as_bytes().to_vec()
        };

        record.resize();
        output_file.write(&record.as_u8_array()).unwrap();

        counter += 1;
        if counter % 40_000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }

    Ok(())
}

pub fn exceeds_database_size(entry_position : u64) -> bool {
    let metadata = fs::metadata(DATABASE_FILE).unwrap();
    if metadata.len() > entry_position { false } else { true }
}

pub fn records_from_entries(entries: Vec<u32>) -> Option<Vec<Record>> {
    let mut f = File::open(DATABASE_FILE).unwrap();
    let mut buffer : Vec<u8>;
    let mut record : Record;
    let mut returned_records : Vec<Record> = Vec::new();

    let mut current_offset : i64 = f.seek(SeekFrom::Start(0)).unwrap() as i64;

    for entry in entries {
        current_offset = f.seek(SeekFrom::Current(((entry - 1) * record::DATA_ENTRY_SIZE as u32) as i64 - current_offset)).unwrap() as i64;
        record = Record::default();

        if exceeds_database_size(current_offset as u64) {
            continue; // Checks if there is that many workers in the database
        }

        for (i, bytes) in record::RECORD_SIZES.iter().enumerate() {

            buffer = vec![0; *bytes as usize];
            f.read_exact(&mut buffer).unwrap();

            let text : &str = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0));

            match i {
                0 => record.nome = text.as_bytes().to_vec(),
                1 => record.id = text.as_bytes().to_vec(),
                2 => record.cpf = text.as_bytes().to_vec(),
                3 => record.descricao_cargo = text.as_bytes().to_vec(),
                4 => record.orgao_exercicio = text.as_bytes().to_vec(),
                5 => record.remuneracao_basica_bruta_rs = text.as_bytes().to_vec(),
                6 => record.gratificacao_natalina_rs = text.as_bytes().to_vec(),
                7 => record.ferias_rs = text.as_bytes().to_vec(),
                8 => record.outras_remuneracoes_eventuais_rs = text.as_bytes().to_vec(),
                9 => record.irrf_rs = text.as_bytes().to_vec(),
                10 => record.pss_rgps_rs = text.as_bytes().to_vec(),
                11 => record.demais_deducoes_rs = text.as_bytes().to_vec(),
                12 => record.remuneracao_apos_deducoes_obrigatorias_rs = text.as_bytes().to_vec(),
                13 => record.total_verbas_indenizatorias_rs = text.as_bytes().to_vec(),
                14 => record.data_inicio_afastamento = text.as_bytes().to_vec(),
                15 => record.data_termino_afastamento = text.as_bytes().to_vec(),
                16 => record.jornada_trabalho = text.as_bytes().to_vec(),
                17 => record.data_ingresso_cargo = text.as_bytes().to_vec(),
                18 => record.data_ingresso_orgao = text.as_bytes().to_vec(),
                _ => println!("Error!!")
            }
        }

        returned_records.push(record);
        current_offset += record::DATA_ENTRY_SIZE as i64;
    }

    Some(returned_records)
}
