use super::Table;

/// # `query`
/// Takes a given `Table` and the query to parse as `String`
pub fn query(table: &mut Table, query: &String) -> Result<(), &'static str> {
    // === GET ===
    if query.starts_with("GET") {
        // get everything after the word GET and prepare it for parsing
        let args = query[3..]
            .split("OF")
            .map(|arg| {
                arg.split(',')
                    .map(|subarg| subarg.to_string().replace(" ", ""))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        if args.len() < 2 {
            return Err("The given query is incorrect");
        }
        // Search the table for results then print it
        let result = table.get(&args[0], &args[1]);
        println!("{:?}", result);
        return Ok(());

    // === SET ===
    } else if query.starts_with("SET") {
        // Get everything after SET and prepare it for parsing
        let mut args = query[3..]
            .split_terminator("OF")
            .map(|data| data.replace(" ", ""));

        // Get list of columns to change
        let to_change: Vec<(String, String)> = if let Some(to_change) = args.next() {
            to_change
                .split_terminator(',')
                .map(|data| {
                    let data_pair: Vec<&str> = data.split('=').collect();
                    (data_pair[0].to_string(), data_pair[1].to_string())
                })
                .collect()
        } else {
            return Err("Error parsing column part of SET command");
        };

        // Get row name
        if let Some(row) = args.next() {
            // Change value in table
            table.set(row, to_change)?;
        } else {
            return Err("Error parsing row name part of  SET command");
        }
    // === DELETE ===
    } else if query.starts_with("DELETE") {
        table.remove_row(&query[6..].replace(" ", "").to_string())?;
        return Ok(());

    // === INSERT ===
    } else if query.starts_with("INSERT") {
        // Prepare data after INSERT
        let mut args = query[6..]
            .split_terminator(':')
            .map(|data| data.replace(" ", ""));

        let row_name = if let Some(row_name) = args.next() {
            row_name.to_string()
        } else {
            return Err("Error parsing row name of INSERT command");
        };
        let content: Vec<String> = if let Some(content) = args.next() {
            content
                .split_terminator(',')
                .map(|data| data.to_string())
                .collect()
        } else {
            return Err("Error parsing column part of INSERT command");
        };

        table.new_row(row_name, content)?;
    } else if query.starts_with("SAVE") {
        table.save()?;
        println!("Saved!");
        return Ok(());
    } else if query.starts_with("ABORT") {
        println!("Quitted without saving table");
        std::process::exit(0);
    }

    Err("No such command")
}
