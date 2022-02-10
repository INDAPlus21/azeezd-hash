use super::DataItem;
use crate::hash_map::Map;
use std::fs::*;
use std::io::{BufRead, Write};

/// # `SearchResult`
/// Type that mostly used when returning results from requesting data from the table
type SearchResult = Vec<(String, Option<Vec<DataItem>>)>;

/// # `Table`
/// A structure that holds represents a Table. The `Table` contains a header which gives names to each column.
/// It also holds a `Map` which correlates a row name as `String` to a vector of `DataItem`s which represents a row in a table.
#[derive(Debug)]
pub struct Table {
    path: String,
    header_idx_map: Map<String, usize>,
    header: Vec<(String, DataItem)>,
    map: Map<String, Vec<DataItem>>,
}

impl Table {
    /// # `new`
    /// Takes a given `String` of a file path to a .csv file containg a table that follow this Table's rules (Read README)
    /// a `Result<Table, &'static str>` where `Ok(Table)` is return if no errors arise (such as giving columns with the same name).
    /// If there is an error it is returned as `Err()`
    pub fn new(path: String) -> Result<Table, &'static str> {
        let mut header_idx_map: Map<String, usize> = Map::new();
        let mut header: Vec<(String, DataItem)> = Vec::new();

        // Get table file
        if let Ok(file) = File::open(path.clone()) {
            let mut lines = std::io::BufReader::new(file).lines();

            // Read header of table
            if let Some(Ok(hdr)) = lines.next() {
                // Read columns of header and prepare them for parsing
                let columns = hdr.split(',').map(|data| data.trim().split_at(2));

                // Go through each column and parse them into the header in this struct
                for col in columns.enumerate() {
                    let col_data = match col.1 .0 {
                        "w:" => (col.1 .1.to_string(), DataItem::Word(String::new())),
                        "b:" => (col.1 .1.to_string(), DataItem::Boolean(false)),
                        "u:" => (col.1 .1.to_string(), DataItem::UInteger(0)),
                        "i:" => (col.1 .1.to_string(), DataItem::Integer(0)),
                        "f:" => (col.1 .1.to_string(), DataItem::Float(0.0)),
                        _ => return Err("Error while reading header"),
                    };

                    header.push(col_data);
                    header_idx_map.insert(col.1 .1.to_string(), col.0)?;
                }
            }

            let mut table = Table {
                path: path.clone(),
                header_idx_map,
                header: Vec::from(header),
                map: Map::new(),
            };

            // Go through each row and inserting  their data into this struct's map
            for line in lines {
                if let Ok(line) = line {
                    let mut col_data = line.split(',').map(|data| data.trim().to_string());
                    table.new_row(
                        if let Some(item) = col_data.next() {
                            item
                        } else {
                            break;
                        },
                        col_data.collect::<Vec<String>>(),
                    )?;
                }
            }

            return Ok(table);
        }

        Err("File not found")
    }

    /// # `save`
    /// Saves the current table to the file it was opened from.
    pub fn save(&mut self) -> Result<(), &'static str> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.clone());

        if file.is_err() {
            return Err("Error while opening the file to save");
        }
        let mut file = file.unwrap();

        // Entire file saved here first
        let mut buffer = String::new();

        // Write header
        for column in self.header.iter() {
            match column.1 {
                DataItem::Boolean(_) => buffer.push_str("b:"),
                DataItem::Word(_) => buffer.push_str("w:"),
                DataItem::UInteger(_) => buffer.push_str("u:"),
                DataItem::Integer(_) => buffer.push_str("i:"),
                DataItem::Float(_) => buffer.push_str("f:"),
            }
            buffer.push_str(format!("{},", column.0).as_str());
        }
        buffer.pop(); // remove the last ,

        // Get rows, iterate through them and save their data to the table
        let keys = self.map.keys().clone();
        for key in keys {
            buffer.push_str(format!("\n{}", key).as_str());
            if let Some(row) = self.map.get(key) {
                for data in row {
                    buffer.push_str(format!(",{}", data.to_string()).as_str());
                }
            }
        }

        if let Err(_) = file.write(buffer.as_bytes()) {
            return Err("Error writing to file");
        }

        Ok(())
    }

    /// # `get`
    /// Takes a `&[String]` representing the name columns to extract and another `&[String]` representing which rows to extract those columns from.
    /// This returns a `SearchResult` which is a vector containing tuples of `(String, Option<Vec<DataItem>>)` in which string represents the row name
    /// and the `Option<Vec<DataItem>>` represents the extract column data from each row, if it exists.
    pub fn get(&mut self, columns: &[String], rows: &[String]) -> SearchResult {
        let mut result: SearchResult = SearchResult::new();

        let mut col_idx: Vec<usize>;
        if columns.contains(&"*".to_string()) && columns.len() == 1 {
            col_idx = (0..self.header.len()).into_iter().collect();
        } else {
            // Get indices of columns
            col_idx = Vec::with_capacity(columns.len());
            for col in columns {
                if let Some(idx) = self.header_idx_map.get(col.to_string()) {
                    col_idx.push(idx);
                }
            }
        }

        let rows = if rows.contains(&"*".to_string()) && rows.len() == 1 {
            (*self.map.keys()).clone()
        } else {
            rows.to_vec()
        };
        let size = rows.len();
        // Fetch columns, row by row (as requested)
        for row in rows.into_iter() {
            if let Some(r) = self.map.get(row.to_string()) {
                let mut row_res: Vec<DataItem> = Vec::with_capacity(size);
                for idx in col_idx.iter() {
                    row_res.push(r[*idx].clone());
                }
                result.push((row.to_string(), Some(row_res)));
            } else {
                result.push((row.to_string(), None));
            }
        }

        result
    }

    /// # `set`
    /// Takes a given row name as `String` and a vector `Vec<(String, String)>` containing pairs of header name and what data they should change to.
    pub fn set(
        &mut self,
        row_name: String,
        content: Vec<(String, String)>,
    ) -> Result<(), &'static str> {
        // Get Row
        if let Some(mut row) = self.map.get(row_name.clone()) {
            // Every content given to change
            for item in content.iter() {
                // Get index of header to change
                if let Some(idx) = self.header_idx_map.get(item.0.to_string()) {
                    // Get content to set the row data to
                    if let Some(current_content) = content.get(idx) {
                        // Get header data type
                        if let Some(value) = self.header.get(idx) {
                            let value = match value.1 {
                                DataItem::Boolean(_) => DataItem::Boolean(
                                    if let Ok(value) = current_content.1.parse::<bool>() {
                                        value
                                    } else {
                                        return Err("Error parsing value as Boolean");
                                    },
                                ),
                                DataItem::Float(_) => DataItem::Float(
                                    if let Ok(value) = current_content.1.parse::<f32>() {
                                        value
                                    } else {
                                        return Err("Error parsing value as Float");
                                    },
                                ),
                                DataItem::UInteger(_) => DataItem::UInteger(
                                    if let Ok(value) = current_content.1.parse::<u32>() {
                                        value
                                    } else {
                                        return Err("Error parsing value as UInteger");
                                    },
                                ),
                                DataItem::Integer(_) => DataItem::Integer(
                                    if let Ok(value) = current_content.1.parse::<i32>() {
                                        value
                                    } else {
                                        return Err("Error parsing value as Integer");
                                    },
                                ),
                                DataItem::Word(_) => DataItem::Word(current_content.1.to_string()),
                            };
                            row[idx] = value;
                        }
                    }
                }
            }
            self.map.set(row_name, row)?;
        }

        Ok(())
    }

    /// # `new_row`
    /// Takes a row name as `&String` and its content as `Vec<String>` and inserts that row into the table.
    /// This then returns `Result<(), &'static str>` where `OK(())` is if the item is inserted, otherwise `Err()` with the error.
    /// The content `Vec` must contain the content in order in which they appear in the header.
    /// That is if the header has [UInteger, Boolean, String] then the content `Vec` must be in that order, otherwise `Err()` is returned
    /// and the row is not inserted.
    pub fn new_row(&mut self, row_name: String, content: Vec<String>) -> Result<(), &'static str> {
        // Incorrect row size check
        if content.len() != self.header.len() {
            return Err("Incorrect amount of column data given");
        }

        let mut converted_data: Vec<DataItem> = Vec::with_capacity(content.len());
        // Incorrect types in row check
        for idx in 0..content.len() {
            if let Some(value) = self.header.get(idx) {
                if let Some(item) = content.get(idx) {
                    match value.1 {
                        DataItem::Boolean(_) => converted_data.push(DataItem::Boolean(
                            if let Ok(value) = item.parse::<bool>() {
                                value
                            } else {
                                return Err("Error parsing value as Boolean");
                            },
                        )),
                        DataItem::Float(_) => converted_data.push(DataItem::Float(
                            if let Ok(value) = item.parse::<f32>() {
                                value
                            } else {
                                return Err("Error parsing value as Float");
                            },
                        )),
                        DataItem::UInteger(_) => converted_data.push(DataItem::UInteger(
                            if let Ok(value) = item.parse::<u32>() {
                                value
                            } else {
                                return Err("Error parsing value as UInteger");
                            },
                        )),
                        DataItem::Integer(_) => converted_data.push(DataItem::Integer(
                            if let Ok(value) = item.parse::<i32>() {
                                value
                            } else {
                                return Err("Error parsing value as Integer");
                            },
                        )),
                        DataItem::Word(_) => converted_data.push(DataItem::Word(
                            if let Some(content) = content.get(idx) {
                                content.clone()
                            } else {
                                return Err("Error parsing value as Word");
                            },
                        )),
                    }
                }
            }
        }

        self.map.insert(row_name.to_string(), converted_data)
    }

    /// # `remove_row`
    /// Takes a row name and removes it. Returns `Ok(Vec<DataItem>)` containing the row removed.
    pub fn remove_row(&mut self, row_name: &String) -> Result<Vec<DataItem>, &'static str> {
        self.map.remove(row_name.clone())
    }
}
