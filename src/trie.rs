use std::collections::HashMap;
use std::{error, io::{Read, Write, Seek, SeekFrom}, fs::OpenOptions};
use std::mem::transmute;

#[derive(Debug, Default)]
pub struct Node {
    chars: HashMap<char, u32>,
    val: Vec<u32>,
    address: u32
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
                nodes: vec!(Node::default())
            },
            root: 0,
        }
    }

    pub fn add(&mut self, string: String, val: u32) {

        // Adiciona o novo valor
        let mut node = self.root;
        for c in string.chars() {
            let len = self.nodes.nodes.len(); // Prevent extra borrowing

            if !self.nodes.nodes[node as usize].chars.contains_key(&c) { // Se não contem aquela chave
                self.nodes.nodes[node as usize].chars.insert(c, len as u32); // Não precisa diminuir 1, pois será aumentado o tamanho

                self.nodes.nodes.push(Node::default());
                node = (self.nodes.nodes.len() - 1) as u32;
            } else {
                node = *self.nodes.nodes[node as usize].chars.get(&c).unwrap();
            }
        }
        self.nodes.nodes[node as usize].val.push(val);

    }

    pub fn save_to_file(&mut self, filename : &str) -> Result<(), Box<error::Error>> {
        let mut output_file = OpenOptions::new().write(true)
                                                .truncate(true)
                                                .create(true)
                                                .open(filename)?;

        // Fakes the initial file creation, calculating the byte address
        let mut counter : u32 = 0;
        for node in self.nodes.nodes.iter_mut() {

            node.address = counter; // We will write this node at this point, so we use the old value for the counter

            //let mut parsed_node : Vec<u8> = Vec::new();

            // parsed_node.push(node.val.len() as u8);
            counter += 1; // 1 byte for the node value vector length (quantity of values)

            // for val in node.val.iter() {
            //     let bytes: [u8; 4] = unsafe { transmute(val.to_le()) };
            //     parsed_node.append(&mut bytes.to_vec());
            // }
            counter += node.val.len() as u32 * 4; // 4 bytes for each entry in the node value vector length

            // parsed_node.push(node.chars.len() as u8);
            counter += 1; // 1 byte for the node characters vector length (quantity of children)

            // for (key, _value) in node.chars.iter() {
            //     parsed_node.push(*key as u8);
            //     let node_address: [u8; 4] = unsafe { transmute(self.nodes.nodes[*value as usize].address.to_le()) };
            //     parsed_node.append(&mut node_address.to_vec());
            // }
            counter += node.chars.len() as u32 * 5; // 1 byte for the value of the char and 4 bytes for the address for each of the mapped chars

        }

        // Print the actual file, with the proper byte address
        for node_index in 0 .. self.nodes.nodes.len() {
            let mut parsed_node : Vec<u8> = Vec::new();

            // Append the length of the vector with the value of the node
            parsed_node.push(self.nodes.nodes[node_index].val.len() as u8);

            // Append the vector with the value of the node
            for val in self.nodes.nodes[node_index].val.iter() {
                let bytes: [u8; 4] = unsafe { transmute(val.to_le()) };
                parsed_node.append(&mut bytes.to_vec());
            }

            // Append the quantity of children
            parsed_node.push(self.nodes.nodes[node_index].chars.len() as u8);

            // Append the children letter, and 4 bytes which will hold their address in the disk in the future
            for (key, value) in self.nodes.nodes[node_index].chars.iter() {
                parsed_node.push(*key as u8);

                let node_address: [u8; 4] = unsafe { transmute(self.nodes.nodes[*value as usize].address.to_le()) };
                parsed_node.append(&mut node_address.to_vec());
            }

            output_file.write(&parsed_node)?;
        }

        Ok(())
    }

    pub fn at_from_file(string : &str, filename: &str) -> Result<Option<Vec<u32>>, Box<error::Error>> {
        let mut input_file = OpenOptions::new().read(true)
                                               .open(filename)?;

        if string.len() > 0 {
            for character in string.chars() {

                // 1st, we jump the values stored in it
                let mut values_len = vec![0; 1];
                input_file.read_exact(&mut values_len)?;
                input_file.read_exact(&mut vec![0; values_len[0] as usize * 4 as usize])?;

                // 2nd, we read the quantity of children
                let mut children_len = vec![0; 1];
                input_file.read_exact(&mut children_len)?;

                // 3rd, we search for the place we should seek for in the file
                let mut found = false;
                for _ in 0..children_len[0] {
                    let mut mapped_char = vec![0; 1];
                    let mut mapped_address = vec![0; 4];
                    input_file.read_exact(&mut mapped_char)?;
                    input_file.read_exact(&mut mapped_address)?;

                    if mapped_char[0] as char == character { // Found the future address, need to parse it
                        let offset : u32 = ((mapped_address[0] as u32) << 0) + ((mapped_address[1] as u32) << 8)
                                         + ((mapped_address[2] as u32) << 16) + ((mapped_address[3] as u32) << 24);
                        input_file.seek(SeekFrom::Start(offset as u64)).unwrap();
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Ok(None)
                } // Else, I'm already in the new place to search for, 'cause I file-seeked to the new position
            }
            // Fetch the values and return it
            let mut values_len = vec![0; 1];
            input_file.read_exact(&mut values_len)?;

            let mut values = vec![0; values_len[0] as usize * 4];
            input_file.read_exact(&mut values)?;

            let mut parsed_values : Vec<u32> = Vec::new();
            for x in 0..values_len[0] {
                let parsed_value : u32 = ((values[(x*4 + 0) as usize] as u32) << 0) + ((values[(x*4 + 1) as usize] as u32) << 8)
                                       + ((values[(x*4 + 2) as usize] as u32) << 16) + ((values[(x*4 + 3) as usize] as u32) << 24);
                parsed_values.push(parsed_value);
            }

            return Ok(Some(parsed_values));
        }

        Ok(None)
    }
}
