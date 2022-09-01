use std::collections::HashMap;

#[derive(Debug)]
pub struct Assembly {
    pub label_table : HashMap<String, usize>,
    pub tokens : Vec<&'static str>
}

pub fn parse_asm(file : &'static str) -> Assembly {
    let split = file
        .split_once("/// END COMPILER GENERATED LABEL TABLE ///")
        .expect("Failed to parse label table - corrupted binary?");

    let label_table_raw : Vec<(usize, &str)> = split.0.split_whitespace().enumerate().collect();
    let label_table = parse_label_table(label_table_raw);
    let tokens = split.1.split_whitespace().collect::<Vec<&'static str>>();

    Assembly {
        label_table,
        tokens
    }
}

fn parse_label_table(table : Vec<(usize, &str)>) -> HashMap<String, usize> {
    let mut parsed : HashMap<String, usize> = HashMap::new();
    let mut key : &str = "";

    for (index, token) in table {
        if index % 2 == 0 {
            key = token
        } else {
            let usize = token.parse::<usize>().expect("Failed to parse label table - corrupted binary?");
            parsed.insert(key.to_owned(), usize);
        }
    }

    parsed
}