use parser;
use record;
use std::collections::HashMap;
use std::mem::transmute;
use std::{
    error,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    str,
};

#[derive(Debug, Default)]
pub struct Node {
    chars: HashMap<char, u32>,
    val: Vec<u32>,
    address: u32,
}

#[derive(Debug, Default)]
pub struct Trie {
    nodes: Arena,
    root: u32,
}

#[derive(Debug, Default)]
pub struct Arena {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            nodes: Arena {
                nodes: vec![Node::default()],
            },
            root: 0,
        }
    }

    pub fn new_from_database(
        trie_file: String,
        record_index: usize,
    ) -> Result<(), Box<error::Error>> {
        let mut trie = Trie::new();
        let mut f = File::open(parser::DATABASE_FILE)?;
        let mut buffer: Vec<u8>;
        let mut record = record::Record::default();
        let mut record_counter = 0;

        while !parser::exceeds_database_size(record_counter * record::DATA_ENTRY_SIZE as u64) {
            //println!("{}", record_counter);
            for (i, bytes) in record::RECORD_SIZES.iter().enumerate() {
                buffer = vec![0; *bytes as usize];
                f.read_exact(&mut buffer).unwrap();

                let text: &str = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0));

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
                    12 => {
                        record.remuneracao_apos_deducoes_obrigatorias_rs = text.as_bytes().to_vec()
                    }
                    13 => record.total_verbas_indenizatorias_rs = text.as_bytes().to_vec(),
                    14 => record.data_inicio_afastamento = text.as_bytes().to_vec(),
                    15 => record.data_termino_afastamento = text.as_bytes().to_vec(),
                    16 => record.jornada_trabalho = text.as_bytes().to_vec(),
                    17 => record.data_ingresso_cargo = text.as_bytes().to_vec(),
                    18 => record.data_ingresso_orgao = text.as_bytes().to_vec(),
                    _ => println!("Error!!"),
                }
            }

            let value = record.get(record_index);
            let split : Vec<&str> = value.split_whitespace().collect();

            for i in 0..split.len() + 1 {
                for j in i+1..split.len() + 1 {
                    trie.add(split[i..j].join(" "), record_counter as u32 + 1); // Add each of the words
                }
            }

            record_counter += 1;
        }

        if let Err(err) = trie.save_to_file(&trie_file) {
            println!("Error saving the trie to a file: {}", err);
        }

        Ok(())
    }

    pub fn new_from_file(trie_file: String) -> Result<Trie, Box<error::Error>> {
        let mut trie = Trie {
            nodes: Arena { nodes: Vec::new() },
            root: 0,
        };

        let mut f = File::open(&trie_file)?;
        let metadata = fs::metadata(&trie_file)?.len();

        while f.seek(SeekFrom::Current(0))? < metadata {
            let mut node: Node = Node::default();

            // 1st, we catch the values stored in it
            let mut values_len = vec![0; 3];
            f.read_exact(&mut values_len)?;

            let mut values =
                vec![0; (values_len[0] as u32 + ((values_len[1] as u32) << 8) + ((values_len[2] as u32) << 16)) as usize * 4];
            let mut node_values: Vec<u32> = Vec::new();
            f.read_exact(&mut values)?;

            for x in 0..values.len() / 4 {
                node_values.push(
                    ((values[x * 4 + 0] as u32) << 0)
                        + ((values[x * 4 + 1] as u32) << 8)
                        + ((values[x * 4 + 2] as u32) << 16)
                        + ((values[x * 4 + 3] as u32) << 24),
                );
            }
            node.val = node_values;

            // 2nd, we read the quantity of children
            let mut children_len = vec![0; 1];
            f.read_exact(&mut children_len)?;

            // 3rd, we read the characters
            for _ in 0..children_len[0] {
                let mut mapped_char = vec![0; 1];
                let mut mapped_arena_position = vec![0; 4];
                let mut _mapped_address = vec![0; 4];

                f.read_exact(&mut mapped_char)?;
                f.read_exact(&mut mapped_arena_position)?;
                f.read_exact(&mut _mapped_address)?;

                node.chars.insert(
                    mapped_char[0] as char,
                    ((mapped_arena_position[0] as u32) << 0)
                        + ((mapped_arena_position[1] as u32) << 8)
                        + ((mapped_arena_position[2] as u32) << 16)
                        + ((mapped_arena_position[3] as u32) << 24),
                );
            }

            trie.nodes.nodes.push(node);
        }

        trie.save_to_file(&trie_file)?;

        return Ok(trie);
    }

    pub fn add(&mut self, string: String, val: u32) {
        // Adiciona o novo valor
        let mut node = self.root;
        for c in string.chars() {
            let len = self.nodes.nodes.len(); // Prevent extra borrowing

            if !self.nodes.nodes[node as usize].chars.contains_key(&c) {
                // Se não contem aquela chave
                self.nodes.nodes[node as usize].chars.insert(c, len as u32); // Não precisa diminuir 1, pois será aumentado o tamanho

                self.nodes.nodes.push(Node::default());
                node = (self.nodes.nodes.len() - 1) as u32;
            } else {
                node = *self.nodes.nodes[node as usize].chars.get(&c).unwrap();
            }
        }
        self.nodes.nodes[node as usize].val.push(val);
    }

    pub fn save_to_file(&mut self, filename: &str) -> Result<(), Box<error::Error>> {
        let mut output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)?;

        // Fakes the initial file creation, calculating the byte address
        let mut counter: u32 = 0;
        for node in self.nodes.nodes.iter_mut() {
            node.address = counter; // We will write this node at this point, so we use the old value for the counter

            //let mut parsed_node : Vec<u8> = Vec::new();

            // parsed_node.push(self.nodes.nodes[node_index].val.len() as u8);
            // parsed_node.push((self.nodes.nodes[node_index].val.len() >> 8) as u8)
            counter += 3; // 3 bytes for the node value vector length (quantity of values)

            // for val in self.nodes.nodes[node_index].val.iter() {
            //     let bytes: [u8; 4] = unsafe { transmute(val.to_le()) };
            //     parsed_node.append(&mut bytes.to_vec());
            // }
            counter += node.val.len() as u32 * 4; // 4 bytes for each entry in the node value vector length

            // parsed_node.push(self.nodes.nodes[node_index].chars.len() as u8);
            counter += 1; // 1 byte for the node characters vector length (quantity of children)

            // for (key, value) in self.nodes.nodes[node_index].chars.iter() {
            //     parsed_node.push(*key as u8);
            //     let node_address: [u8; 4] = unsafe { transmute(self.nodes.nodes[*value as usize].address.to_le()) };
            //     parsed_node.append(&mut node_address.to_vec());
            // }
            counter += node.chars.len() as u32 * 9; // 1 byte for the value of the char, 4 bytes for their arena position and 4 bytes for the address for each of the mapped chars
        }

        // Print the actual file, with the proper byte address
        for node_index in 0..self.nodes.nodes.len() {
            let mut parsed_node: Vec<u8> = Vec::new();

            // Append the length of the vector with the value of the node
            parsed_node.push(self.nodes.nodes[node_index].val.len() as u8);
            parsed_node.push((self.nodes.nodes[node_index].val.len() >> 8) as u8);
            parsed_node.push((self.nodes.nodes[node_index].val.len() >> 16) as u8);

            // Append the vector with the value of the node
            for val in self.nodes.nodes[node_index].val.iter() {
                let bytes: [u8; 4] = unsafe { transmute(val.to_le()) };
                parsed_node.append(&mut bytes.to_vec());
            }

            // Append the quantity of children
            parsed_node.push(self.nodes.nodes[node_index].chars.len() as u8);

            // Append the children letter, their position in the arena and 4 bytes which hold their address in the disk in the future
            for (key, value) in self.nodes.nodes[node_index].chars.iter() {
                parsed_node.push(*key as u8);

                let arena_index: [u8; 4] = unsafe { transmute(value.to_le()) };
                parsed_node.append(&mut arena_index.to_vec());

                let node_address: [u8; 4] =
                    unsafe { transmute(self.nodes.nodes[*value as usize].address.to_le()) };
                parsed_node.append(&mut node_address.to_vec());
            }

            output_file.write(&parsed_node)?;
        }

        Ok(())
    }

    pub fn at_from_file(
        string: &str,
        filename: &str,
        prefix_search: bool,
    ) -> Result<Option<Vec<u32>>, Box<error::Error>> {
        let mut input_file = OpenOptions::new().read(true).open(filename)?;

        if string.len() > 0 {
            for character in string.chars() {
                // 1st, we jump the values stored in it
                let mut values_len = vec![0; 3];
                input_file.read_exact(&mut values_len)?;
                input_file.read_exact(&mut vec![
                    0;
                    (values_len[0] as u32 + ((values_len[1] as u32) << 8) + ((values_len[2] as u32) << 16))
                        as usize
                        * 4 as usize
                ])?;

                // 2nd, we read the quantity of children
                let mut children_len = vec![0; 1];
                input_file.read_exact(&mut children_len)?;

                // 3rd, we search for the place we should seek for in the file
                let mut found = false;
                for _ in 0..children_len[0] {
                    let mut mapped_char = vec![0; 1];
                    let mut _mapped_arena_position = vec![0; 4];
                    let mut mapped_address = vec![0; 4];
                    input_file.read_exact(&mut mapped_char)?;
                    input_file.read_exact(&mut _mapped_arena_position)?;
                    input_file.read_exact(&mut mapped_address)?;

                    if mapped_char[0] as char == character {
                        // Found the future address, need to parse it
                        let offset: u32 = ((mapped_address[0] as u32) << 0)
                            + ((mapped_address[1] as u32) << 8)
                            + ((mapped_address[2] as u32) << 16)
                            + ((mapped_address[3] as u32) << 24);
                        input_file.seek(SeekFrom::Start(offset as u64)).unwrap();
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Ok(None);
                } // Else, I'm already in the new place to search for, 'cause I file-seeked to the new position
            }

            if prefix_search {
                // I need to fetch all the nodes behind me
                let mut queue: Vec<u32> =
                    vec![input_file.seek(SeekFrom::Current(0)).unwrap() as u32];
                let mut parsed_values: Vec<u32> = Vec::new();

                while queue.len() > 0 {
                    // Retira um nodo por vez da fila, e pega o seu valor
                    let offset: u32 = queue.remove(0);

                    input_file.seek(SeekFrom::Start(offset as u64)).unwrap();

                    // 1st, we retrieve the values and fill the parsed_values array
                    let mut values_len = vec![0; 3];
                    input_file.read_exact(&mut values_len)?;

                    let mut values = vec![
                        0;
                        (values_len[0] as u32 + ((values_len[1] as u32) << 8) + ((values_len[2] as u32) << 16))
                            as usize
                            * 4 as usize
                    ];
                    input_file.read_exact(&mut values)?;

                    for x in 0..values.len() / 4 {
                        let parsed_value: u32 = ((values[(x as usize * 4 + 0) as usize] as u32)
                            << 0)
                            + ((values[(x as usize * 4 + 1) as usize] as u32) << 8)
                            + ((values[(x as usize * 4 + 2) as usize] as u32) << 16)
                            + ((values[(x as usize * 4 + 3) as usize] as u32) << 24);
                        parsed_values.push(parsed_value);
                    }

                    // 2nd, we read the quantity of children
                    let mut children_len = vec![0; 1];
                    input_file.read_exact(&mut children_len)?;

                    // 3rd, we search for the places we should still seek for in the file
                    for _ in 0..children_len[0] {
                        let mut _mapped_char = vec![0; 1];
                        let mut _mapped_arena_position = vec![0; 4];
                        let mut mapped_address = vec![0; 4];
                        input_file.read_exact(&mut _mapped_char)?;
                        input_file.read_exact(&mut _mapped_arena_position)?;
                        input_file.read_exact(&mut mapped_address)?;

                        // Found the future address, need to parse it
                        let offset: u32 = ((mapped_address[0] as u32) << 0)
                            + ((mapped_address[1] as u32) << 8)
                            + ((mapped_address[2] as u32) << 16)
                            + ((mapped_address[3] as u32) << 24);
                        queue.push(offset);
                    }
                }

                return Ok(Some(parsed_values));
            } else {
                // I only need to fetch myself
                // Fetch the values and return it
                let mut values_len = vec![0; 3];
                input_file.read_exact(&mut values_len)?;

                let mut values = vec![
                    0;
                    (values_len[0] as u32 + ((values_len[1] as u32) << 8) + ((values_len[2] as u32) << 16))
                        as usize
                        * 4 as usize
                ];
                input_file.read_exact(&mut values)?;

                let mut parsed_values: Vec<u32> = Vec::new();
                for x in 0..values.len() / 4 {
                    let parsed_value: u32 = ((values[(x as usize * 4 + 0) as usize] as u32) << 0)
                        + ((values[(x as usize * 4 + 1) as usize] as u32) << 8)
                        + ((values[(x as usize * 4 + 2) as usize] as u32) << 16)
                        + ((values[(x as usize * 4 + 3) as usize] as u32) << 24);
                    parsed_values.push(parsed_value);
                }

                return Ok(Some(parsed_values));
            }
        }

        Ok(None)
    }
}
