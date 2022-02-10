mod hash_map;
mod tools;

use tools::{Table, query};
use std::env::args;

fn main() {
    let mut args = args();
    if args.len() < 2 {
        println!("Insufficient amount of arguments given");
        return;
    } 
    if let Ok(mut table) = Table::new(args.nth(1).unwrap()) {
        let input = std::io::stdin();
        let mut buffer = String::new();
        loop {
            input.read_line(&mut buffer);
            query(&mut table, &buffer.trim_end().to_string());
            buffer.clear();
        }
    } else {
        println!("Unable to open table");
    }  
}