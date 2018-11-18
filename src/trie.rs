use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    chars: HashMap<char, usize>,
    val: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct Trie {
    nodes: Arena,
    root: usize,
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

            if !self.nodes.nodes[node].chars.contains_key(&c) { // Se não contem aquela chave
                self.nodes.nodes[node].chars.insert(c, len); // Não precisa diminuir 1, pois será aumentado o tamanho

                self.nodes.nodes.push(Node::default());
                node = self.nodes.nodes.len() - 1;
            } else {
                node = *self.nodes.nodes[node].chars.get(&c).unwrap();
            }
        }
        self.nodes.nodes[node].val.push(val);

    }

    pub fn at(&mut self, string: String) -> Option<Vec<u32>> {
        let mut node = self.node_at(self.root);
        for c in string.chars() {
            if node.chars.contains_key(&c) {
                node = self.node_at(*node.chars.get(&c).unwrap());
            } else {
                return None;
            }
        }

        return Some(node.val.clone());
    }

    fn node_at(&self, index : usize) -> Node {
        Node {
            chars: self.nodes.nodes[index].chars.clone(),
            val: self.nodes.nodes[index].val.clone(),
        }
    }
}
