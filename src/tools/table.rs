use super::DataItem;
use crate::hash_map::Map;

/// # `SearchResult`
/// Type that mostly used when returning results from requesting data from the table
type SearchResult = Vec<(String, Option<Vec<DataItem>>)>;

/// # `Table`
/// A structure that holds represents a Table. The `Table` contains a header which gives names to each column.
/// It also holds a `Map` which correlates a row name as `String` to a vector of `DataItem`s which represents a row in a table.
pub struct Table {
    header_idx_map: Map<String, usize>,
    header: Vec<(String, DataItem)>,
    map: Map<String, Vec<DataItem>>,
}

impl Table {
    /// # `new`
    /// Takes a given `&[(String, DataItem)]` representing the name of each column and what type that column holds then returns
    /// a `Result<Table, &'static str>` where `Ok(Table)` is return if no errors arise (such as giving columns with the same name).
    /// If there is an error it is returned as `Err()`
    pub fn new(header: &[(String, DataItem)]) -> Result<Table, &'static str> {
        let mut header_idx_map: Map<String, usize> = Map::new();

        for col_idx in 0..header.len() {
            if let Err(e) = header_idx_map.insert(header[col_idx].0.clone(), col_idx) {
                println!("{}", e);
                return Err(e);
            }
        }

        Ok(Table {
            header_idx_map,
            header: Vec::from(header),
            map: Map::new(),
        })
    }

    /// # `get`
    /// Takes a `&[String]` representing the name columns to extract and another `&[String]` representing which rows to extract those columns from.
    /// This returns a `SearchResult` which is a vector containing tuples of `(String, Option<Vec<DataItem>>)` in which string represents the row name
    /// and the `Option<Vec<DataItem>>` represents the extract column data from each row, if it exists.
    pub fn get(&self, columns: &[String], rows: &[String]) -> SearchResult {
        let mut result: SearchResult = SearchResult::new();

        // Get indices of columns
        let mut col_idx: Vec<usize> = Vec::with_capacity(columns.len());
        for col in columns {
            if let Some(idx) = self.header_idx_map.get(col.to_string()) {
                col_idx.push(idx);
            }
        }

        // Fetch columns, row by row (as requested)
        for row in rows {
            if let Some(r) = self.map.get(row.to_string()) {
                let mut row_res: Vec<DataItem> = Vec::with_capacity(rows.len());
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
    /// Takes a row name as `&String` and its content as `Vec<DataItem>` and inserts that row into the table.
    /// This then returns `Result<(), &'static str>` where `OK(())` is if the item is inserted, otherwise `Err()` with the error.
    /// The content `Vec` must contain the content in order in which they appear in the header.
    /// That is if the header has [UInteger, Boolean, String] then the content `Vec` must be in that order, otherwise `Err()` is returned
    /// and the row is not inserted.
    pub fn set(&mut self, row_name: &String, content: Vec<DataItem>) -> Result<(), &'static str> {
        // Incorrect row size check
        if content.len() > self.header.len() {
            return Err("There is more content than there exists place");
        }

        // Incorrect types in row check
        for idx in 0..content.len() {
            if !content[idx].same_type_as(&self.header[idx].1) {
                return Err("Types in row do not match those in header");
            }
        }

        self.map.insert(row_name.to_string(), content)
    }
}
