use std::fmt;
use std::str;

pub const NAME_MAX_SIZE: usize = 40;
pub const CPF_MAX_SIZE: usize = 15;
pub const SALARY_MAX_SIZE: usize = 10;
pub const DATA_ENTRY_SIZE :usize = NAME_MAX_SIZE + CPF_MAX_SIZE + SALARY_MAX_SIZE * 10;
pub const RECORD_SIZES : [usize; 12] = [NAME_MAX_SIZE, SALARY_MAX_SIZE, CPF_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE,
                                        SALARY_MAX_SIZE, SALARY_MAX_SIZE, SALARY_MAX_SIZE];

#[derive(PartialOrd, PartialEq, Default)]
pub struct Record {
    pub nome: Vec<u8>,
    pub id: Vec<u8>,
    pub cpf: Vec<u8>,
    pub remuneracao_basica_bruta_rs: Vec<u8>,
    pub gratificacao_natalina_rs: Vec<u8>,
    pub ferias_rs: Vec<u8>,
    pub outras_remuneracoes_eventuais_rs: Vec<u8>,
    pub irrf_rs: Vec<u8>,
    pub pss_rgps_rs: Vec<u8>,
    pub demais_deducoes_rs: Vec<u8>,
    pub remuneracao_apos_deducoes_obrigatorias_rs: Vec<u8>,
    pub total_verbas_indenizatorias_rs: Vec<u8>
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

    pub fn as_u8_array(&mut self) -> Vec<u8> {
        let mut vec : Vec<u8> = Vec::new();
        vec.append(&mut self.nome);
        vec.append(&mut self.id);
        vec.append(&mut self.cpf);
        vec.append(&mut self.remuneracao_basica_bruta_rs);
        vec.append(&mut self.gratificacao_natalina_rs);
        vec.append(&mut self.ferias_rs);
        vec.append(&mut self.outras_remuneracoes_eventuais_rs);
        vec.append(&mut self.irrf_rs);
        vec.append(&mut self.pss_rgps_rs);
        vec.append(&mut self.demais_deducoes_rs);
        vec.append(&mut self.remuneracao_apos_deducoes_obrigatorias_rs);
        vec.append(&mut self.total_verbas_indenizatorias_rs);

        return vec;
    }

    pub fn resize(&mut self){
        self.nome.resize(NAME_MAX_SIZE, 0);
        self.id.resize(SALARY_MAX_SIZE, 0);
        self.cpf.resize(CPF_MAX_SIZE, 0);
        self.remuneracao_basica_bruta_rs.resize(SALARY_MAX_SIZE, 0);
        self.gratificacao_natalina_rs.resize(SALARY_MAX_SIZE, 0);
        self.ferias_rs.resize(SALARY_MAX_SIZE,0);
        self.outras_remuneracoes_eventuais_rs.resize(SALARY_MAX_SIZE, 0);
        self.irrf_rs.resize(SALARY_MAX_SIZE, 0);
        self.pss_rgps_rs.resize(SALARY_MAX_SIZE, 0);
        self.demais_deducoes_rs.resize(SALARY_MAX_SIZE, 0);
        self.remuneracao_apos_deducoes_obrigatorias_rs.resize(SALARY_MAX_SIZE, 0);
        self.total_verbas_indenizatorias_rs.resize(SALARY_MAX_SIZE, 0);
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id do Servidor: {}\n\
                   Nome & CPF do Servidor Público: {} - {}\n\
                   Remuneração Bruta: R$ {}\n\
                   Gratificação Natalina: R${}\n\
                   Ferias: R$ {}\n\
                   Outras remunerações: R$ {}\n\
                   Imposto de Renda: R$ {}\n\
                   Seguridade Social: R$ {}\n\
                   Demais Deducoes: R$ {}\n\
                   Remuneração Após Deduções Obrigatórias (IRRF+PSS): R$ {}\n\
                   Remuneração Provinda de Verbas Indenizatórias: R$ {}",
                   self.get_id(), self.get_name(), self.get_cpf(), self.get_remuneracao_bruta(),
                   self.get_gratificacao_natalina(), self.get_ferias(), self.get_outras_remuneracoes(),
                   self.get_irrf(), self.get_pss(), self.get_demais_reducoes(),
                   self.get_remuneracao_apos_deducoes(), self.get_verbas_indenizatorias())
    }
}
