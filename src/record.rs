use std::{fmt, str};

pub const NAME_MAX_SIZE : usize = 40;
pub const CPF_MAX_SIZE : usize = 15;
pub const SALARY_MAX_SIZE : usize = 10;
pub const DESCRIPTION_MAX_SIZE : usize = 50;
pub const DATA_MAX_SIZE : usize = 12;
pub const DEDICACAO_MAX_SIZE : usize = 20;
pub const DATA_ENTRY_SIZE : usize = NAME_MAX_SIZE + CPF_MAX_SIZE + SALARY_MAX_SIZE * 10
                                    + DESCRIPTION_MAX_SIZE * 2 + DATA_MAX_SIZE * 4
                                    + DEDICACAO_MAX_SIZE;
pub const RECORD_SIZES : [usize; 19] = [NAME_MAX_SIZE, SALARY_MAX_SIZE, CPF_MAX_SIZE,
                                        DESCRIPTION_MAX_SIZE, DESCRIPTION_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE,
                                        DATA_MAX_SIZE, DATA_MAX_SIZE,
                                        DEDICACAO_MAX_SIZE, DATA_MAX_SIZE, DATA_MAX_SIZE];

#[derive(PartialOrd, PartialEq, Default)]
pub struct Record {
    pub nome: Vec<u8>,
    pub id: Vec<u8>,
    pub cpf: Vec<u8>,
    pub descricao_cargo: Vec<u8>,
    pub orgao_exercicio: Vec<u8>,
    pub remuneracao_basica_bruta_rs: Vec<u8>,
    pub gratificacao_natalina_rs: Vec<u8>,
    pub ferias_rs: Vec<u8>,
    pub outras_remuneracoes_eventuais_rs: Vec<u8>,
    pub irrf_rs: Vec<u8>,
    pub pss_rgps_rs: Vec<u8>,
    pub demais_deducoes_rs: Vec<u8>,
    pub remuneracao_apos_deducoes_obrigatorias_rs: Vec<u8>,
    pub total_verbas_indenizatorias_rs: Vec<u8>,
    pub data_inicio_afastamento: Vec<u8>,
    pub data_termino_afastamento: Vec<u8>,
    pub jornada_trabalho: Vec<u8>,
    pub data_ingresso_cargo: Vec<u8>,
    pub data_ingresso_orgao: Vec<u8>
}

impl Record {
    fn get_name(&self) -> &str {
        str::from_utf8(&self.nome).unwrap()
    }

    fn get_id(&self) -> &str {
        str::from_utf8(&self.id).unwrap()
    }

    fn get_cpf(&self) -> &str {
        str::from_utf8(&self.cpf).unwrap()
    }

    fn get_descricao_cargo(&self) -> &str {
        str::from_utf8(&self.descricao_cargo).unwrap()
    }

    fn get_orgao_exercicio(&self) -> &str {
        str::from_utf8(&self.orgao_exercicio).unwrap()
    }

    fn get_remuneracao_bruta(&self) -> &str {
        str::from_utf8(&self.remuneracao_basica_bruta_rs).unwrap()
    }

    fn get_gratificacao_natalina(&self) -> &str {
        str::from_utf8(&self.gratificacao_natalina_rs).unwrap()
    }

    fn get_ferias(&self) -> &str {
        str::from_utf8(&self.ferias_rs).unwrap()
    }

    fn get_outras_remuneracoes(&self) -> &str {
        str::from_utf8(&self.outras_remuneracoes_eventuais_rs).unwrap()
    }

    fn get_irrf(&self) -> &str {
        str::from_utf8(&self.irrf_rs).unwrap()
    }

    fn get_pss(&self) -> &str {
        str::from_utf8(&self.pss_rgps_rs).unwrap()
    }

    fn get_demais_reducoes(&self) -> &str {
        str::from_utf8(&self.demais_deducoes_rs).unwrap()
    }

    fn get_remuneracao_apos_deducoes(&self) -> &str {
        str::from_utf8(&self.remuneracao_apos_deducoes_obrigatorias_rs).unwrap()
    }

    fn get_verbas_indenizatorias(&self) -> &str {
        str::from_utf8(&self.total_verbas_indenizatorias_rs).unwrap()
    }

    fn get_data_inicio_afastamento(&self) -> &str {
        str::from_utf8(&self.data_inicio_afastamento).unwrap()
    }

    fn get_data_termino_afastamento(&self) -> &str {
        str::from_utf8(&self.data_termino_afastamento).unwrap()
    }

    fn get_jornada_trabalho(&self) -> &str {
        str::from_utf8(&self.jornada_trabalho).unwrap()
    }

    fn get_data_ingresso_cargo(&self) -> &str {
        str::from_utf8(&self.data_ingresso_cargo).unwrap()
    }

    fn get_data_ingresso_orgao(&self) -> &str {
        str::from_utf8(&self.data_ingresso_orgao).unwrap()
    }

    pub fn generate_csv_string(&mut self) -> String {
        let mut return_string : String = String::new();

        return_string += &(self.get_name().to_owned() + &";".to_string());
        return_string += &(self.get_descricao_cargo().to_owned() + &";".to_string());
        return_string += &(self.get_orgao_exercicio().to_owned() + &";".to_string());
        return_string += &(self.get_remuneracao_bruta().to_owned() + &";".to_string());
        return_string += &(self.get_gratificacao_natalina().to_owned() + &";".to_string());
        return_string += &(self.get_irrf().to_owned() + &";".to_string());
        return_string += &(self.get_pss().to_owned() + &";".to_string());
        return_string += &(self.get_demais_reducoes().to_owned() + &";".to_string());
        return_string += &(self.get_remuneracao_apos_deducoes().to_owned() + &";".to_string());
        return_string += self.get_verbas_indenizatorias(); // No ';' in the final one
        return_string += "\n";

        return return_string;
    }

    pub fn as_u8_array(&mut self) -> Vec<u8> {
        let mut vec : Vec<u8> = Vec::new();
        vec.append(&mut self.nome);
        vec.append(&mut self.id);
        vec.append(&mut self.cpf);
        vec.append(&mut self.descricao_cargo);
        vec.append(&mut self.orgao_exercicio);
        vec.append(&mut self.remuneracao_basica_bruta_rs);
        vec.append(&mut self.gratificacao_natalina_rs);
        vec.append(&mut self.ferias_rs);
        vec.append(&mut self.outras_remuneracoes_eventuais_rs);
        vec.append(&mut self.irrf_rs);
        vec.append(&mut self.pss_rgps_rs);
        vec.append(&mut self.demais_deducoes_rs);
        vec.append(&mut self.remuneracao_apos_deducoes_obrigatorias_rs);
        vec.append(&mut self.total_verbas_indenizatorias_rs);
        vec.append(&mut self.data_inicio_afastamento);
        vec.append(&mut self.data_termino_afastamento);
        vec.append(&mut self.jornada_trabalho);
        vec.append(&mut self.data_ingresso_cargo);
        vec.append(&mut self.data_ingresso_orgao);

        return vec;
    }

    pub fn resize(&mut self){
        self.nome.resize(NAME_MAX_SIZE, 0);
        self.id.resize(SALARY_MAX_SIZE, 0);
        self.cpf.resize(CPF_MAX_SIZE, 0);
        self.descricao_cargo.resize(DESCRIPTION_MAX_SIZE, 0);
        self.orgao_exercicio.resize(DESCRIPTION_MAX_SIZE, 0);
        self.remuneracao_basica_bruta_rs.resize(SALARY_MAX_SIZE, 0);
        self.gratificacao_natalina_rs.resize(SALARY_MAX_SIZE, 0);
        self.ferias_rs.resize(SALARY_MAX_SIZE,0);
        self.outras_remuneracoes_eventuais_rs.resize(SALARY_MAX_SIZE, 0);
        self.irrf_rs.resize(SALARY_MAX_SIZE, 0);
        self.pss_rgps_rs.resize(SALARY_MAX_SIZE, 0);
        self.demais_deducoes_rs.resize(SALARY_MAX_SIZE, 0);
        self.remuneracao_apos_deducoes_obrigatorias_rs.resize(SALARY_MAX_SIZE, 0);
        self.total_verbas_indenizatorias_rs.resize(SALARY_MAX_SIZE, 0);
        self.data_inicio_afastamento.resize(DATA_MAX_SIZE, 0);
        self.data_termino_afastamento.resize(DATA_MAX_SIZE, 0);
        self.jornada_trabalho.resize(DEDICACAO_MAX_SIZE, 0);
        self.data_ingresso_cargo.resize(DATA_MAX_SIZE, 0);
        self.data_ingresso_orgao.resize(DATA_MAX_SIZE, 0);
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id do Servidor: {}\n\
                   Nome & CPF do Servidor Público: {} - {}\n\
                   Cargo: {}\n\
                   Órgao em Exercício: {}\n\
                   Remuneração Bruta: R$ {}\n\
                   Gratificação Natalina: R${}\n\
                   Ferias: R$ {}\n\
                   Outras remunerações: R$ {}\n\
                   Imposto de Renda: R$ {}\n\
                   Seguridade Social: R$ {}\n\
                   Demais Deducoes: R$ {}\n\
                   Remuneração Após Deduções Obrigatórias (IRRF+PSS): R$ {}\n\
                   Remuneração Provinda de Verbas Indenizatórias: R$ {}\n\
                   Data de Inicio e Termino do Afastamento: {} {}\n\
                   Jornada de Trabalho: {}\n\
                   Data de Ingresso no Cargo: {}\n\
                   Data de Ingresso no Orgao: {}",
                   self.get_id(), self.get_name(), self.get_cpf(), self.get_descricao_cargo(),
                   self.get_orgao_exercicio(),
                   self.get_remuneracao_bruta(), self.get_gratificacao_natalina(),
                   self.get_ferias(), self.get_outras_remuneracoes(), self.get_irrf(),
                   self.get_pss(), self.get_demais_reducoes(),
                   self.get_remuneracao_apos_deducoes(), self.get_verbas_indenizatorias(),
                   if self.data_inicio_afastamento.len() > 1 { self.get_data_inicio_afastamento() } else { "Não está afastado" },
                   if self.data_termino_afastamento.len() > 1 { "até ".to_owned() + self.get_data_termino_afastamento()} else { " ".to_string() },
                   self.get_jornada_trabalho(),
                   self.get_data_ingresso_cargo(), self.get_data_ingresso_orgao())
    }
}
