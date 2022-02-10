use std::ops::Index;

use super::Table;

pub fn query(table: &mut Table, query: &String) -> Result<(), &'static str> {
    if query.starts_with("GET") {
        let args = query[3..]
            .split("OF")
            .map(|arg| {
                arg.split(',')
                    .map(|subarg| subarg.to_string().replace(" ", ""))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let result = table.get(&args[0], &args[1]);
        println!("{:?}", result);
    } else if query.starts_with("SET") {
        let mut args = query[3..]
            .split_terminator("OF")
            .map(|data| data.replace(" ", ""));
        let to_change = args
            .next()
            .unwrap()
            .split_terminator(',')
            .map(|data| {
                let data_pair: Vec<&str> = data.split('=').collect();
                (data_pair[0].to_string(), data_pair[1].to_string())
            })
            .collect();
        let row = args.next().unwrap();
        table.set(row, to_change)?;
    } else if query.starts_with("DELETE") {
        table.remove_row(&query[6..].replace(" ", "").to_string())?;
    } else if query.starts_with("INSERT") {
        let mut args = query[6..]
            .split_terminator(':')
            .map(|data| data.replace(" ", ""));
        let row_name = args.next().unwrap().to_string();
        let content = args
            .next()
            .unwrap()
            .split_terminator(',')
            .map(|data| data.to_string())
            .collect();
        table.new_row(row_name, content)?;
    }

    Err("No such command")
}
