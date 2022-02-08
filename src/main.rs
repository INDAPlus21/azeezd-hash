mod hash_map;
mod tools;

use tools::{Table, DataItem};

fn main() {
    let mut table = Table::new(&[
        ("Age".to_string(), DataItem::UInteger(0)),
        ("Gender".to_string(), DataItem::Boolean(false))
        ]).unwrap();

    table.set(&"Bob".to_string(), vec![DataItem::UInteger(23), DataItem::Boolean(false)]);
    
    println!("{:?}", table.get(&["Age".to_string(), "Gender".to_string()], &["HA".to_string(), "Bob".to_string()]));
}