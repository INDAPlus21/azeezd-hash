mod hash_map;
mod tools;

use tools::{Table, DataItem, query};

fn main() {
    let mut table = Table::new(&[
        ("Age".to_string(), DataItem::UInteger(0)),
        ("Gender".to_string(), DataItem::Word(String::new()))
        ]).unwrap();

        let input = std::io::stdin();
        let mut buffer = String::new();
        loop {
            input.read_line(&mut buffer);
            query(&mut table, &buffer.trim_end().to_string());
            buffer.clear();
        }
}