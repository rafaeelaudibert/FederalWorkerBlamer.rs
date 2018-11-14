extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::error::Error;
use std::io;
use std::process;
use std::time::Instant;
use bincode::{serialize_into, deserialize_from};
use std::fs::File;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Record {
    nome: String,
    id: String,
    cpf: String,
    remuneracao_basica_bruta_rs: String,
    abate_teto_rs: String,
    gratificacao_natalina_rs: String,
    abate_teto_gratificacao_natalina_rs: String,
    ferias_rs: String,
    outras_remuneracoes_eventuais_rs: String,
    irrf_rs: String,
    pss_rgps_rs: String,
    demais_deducoes_rs: String,
    pensao_militar_rs: String,
    fundo_de_saude_rs: String,
    taxa_ocupacao_imovel_funcional_rs: String,
    remuneracao_apos_deducoes_obrigatorias_rs: String,
    verbas_indenizatorias_registradas_sistemas_pesoal_civil_rs: String,
    verbas_indenizatorias_registradas_sistemas_pesoal_militar_rs: String,
    verbas_indenizatorias_desligamento_voluntario_mp_rs: String,
    total_verbas_indenizatorias_rs: String
}

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(io::stdin());
    let _vec: Vec<Record> = Vec::new();
    //let buffer = File::create("fooRelease.bin")?; // New file
    let _f = File::open("foo.bin")?;               // Read file

    // FAST AND UGLY CODE
    for result in rdr.records().map(|r| r.unwrap()) {

        let _record = Record {
            nome: result[4].to_string(),
            id: result[2].to_string(),
            cpf: result[3].to_string(),
            remuneracao_basica_bruta_rs: result[5].to_string(),
            abate_teto_rs: result[7].to_string(),
            gratificacao_natalina_rs: result[9].to_string(),
            abate_teto_gratificacao_natalina_rs: result[11].to_string(),
            ferias_rs: result[13].to_string(),
            outras_remuneracoes_eventuais_rs: result[15].to_string(),
            irrf_rs: result[17].to_string(),
            pss_rgps_rs: result[19].to_string(),
            demais_deducoes_rs: result[21].to_string(),
            pensao_militar_rs: result[23].to_string(),
            fundo_de_saude_rs: result[25].to_string(),
            taxa_ocupacao_imovel_funcional_rs: result[27].to_string(),
            remuneracao_apos_deducoes_obrigatorias_rs: result[29].to_string(),
            verbas_indenizatorias_registradas_sistemas_pesoal_civil_rs: result[31].to_string(),
            verbas_indenizatorias_registradas_sistemas_pesoal_militar_rs: result[33].to_string(),
            verbas_indenizatorias_desligamento_voluntario_mp_rs: result[35].to_string(),
            total_verbas_indenizatorias_rs: result[37].to_string()
        };

        // println!("{:?}", record);
    }

    //  SLOW AND PRETTY CODE
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let _record: Record = result?;
        // println!("{:?}", record);
    }

    //serialize_into(buffer, &vec).unwrap();                        // Write binary to file
    //let vec_read: Vec<Record> = deserialize_from(f).unwrap();     // Read binary from file
    //println!("{:?}", vec_read[0]);                                // Print first position of read binary data
    Ok(())
}

fn main() {
    let before = Instant::now();
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    println!("Time elapsed: {:?}", Instant::now().duration_since(before));
    println!("Finished!")
}
