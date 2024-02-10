use futures::executor::block_on;
use postgres::{NoTls, Row, Column};
use pad::PadStr;

use crate::config::Connection;

const DIVIDER: &str = "|";

struct DisplayRow {
    raw_row: Vec<String>,
    max_width_per_column: Vec<usize>,
    // max_width: usize
}

impl DisplayRow {
    pub fn new(row: &Row) -> DisplayRow {
        let raw_row = row.columns().into_iter().map(|col| {
            let val: Option<String> = row.get(col.name());
            return val.unwrap_or(String::from(""));
        }).collect();

        let widths: Vec<usize> = row.columns().into_iter().map(|x| {
            let val: Option<String> = row.get(x.name());
            if let Some(_i) = val {
                return _i.as_str().len();
            } else {
                return 0;
            }
        }).collect();
        // let width_clone = widths.clone();
        return DisplayRow { 
            // row: Some(row),
            raw_row: raw_row,
            max_width_per_column: widths,
            // max_width: width_clone.iter().sum()
        }
    }

    pub fn for_columns(columns: &[Column]) -> DisplayRow {
        let raw_row = columns.into_iter().map(|col| {
            return col.name().to_string();
        }).collect();


        let widths: Vec<usize> = columns.into_iter().map(|col| {
            return col.name().len();
        }).collect();

        // let width_clone = widths.clone();

        return DisplayRow { 
            raw_row: raw_row,
            max_width_per_column: widths,
            // max_width: width_clone.iter().sum() 
        }
    }
}

fn print(rows: Vec<Row>) -> String {
    if rows.len() <= 0 {
        return String::from("No results returned.");
    } else {
        let divider = String::from(DIVIDER);

        // Create display rows from rows
        // Create disaply row from columns
        // Per column find max width of all display rows
        // Get max width of rows for section dividers
        // print start
        // Render rows with padding and divider
        // Print end

        let mut display_rows: Vec<DisplayRow> = Vec::new();
        for row in &rows {
            display_rows.push(DisplayRow::new(row));
        }
    
        let column_display_row = DisplayRow::for_columns(rows[0].columns());

        // Get max width per column
        // Change this to field on struct
        let column_count = rows[0].columns().len().clone();
        let mut col_widths: Vec<usize> = vec![0; column_count];
        for drow in &display_rows {
            for (i, width) in drow.max_width_per_column.iter().enumerate() {
                if col_widths[i] < *width {
                    col_widths[i] = width.clone();
                }
            }
        }

        // Check column names also
        for (i, col) in column_display_row.raw_row.iter().enumerate() {
            if col_widths[i] < col.len() {
                col_widths[i] = col.len().clone();
            }
        }

        let mut total_width: usize = col_widths.iter().sum();
        total_width = total_width + 2 + col_widths.len() - 1; // Include cell borders

        // print first line that is total with
        let first_line_border = vec!["-"; total_width - 2];
        let mut first_line_vec = vec!["+"];
        let first_line_end = vec!["+"];
        first_line_vec.extend_from_slice(&first_line_border);
        first_line_vec.extend_from_slice(&first_line_end);
        let first_line = first_line_vec.into_iter().collect::<String>();
        // print column line, edges plus for each cell then fill to col_width
        let mut column_line_vec = Vec::new();
        for (i, col) in column_display_row.raw_row.iter().enumerate() {
            column_line_vec.push(divider.clone());
            let cap = col_widths[i];
            let cell = col.clone().pad_to_width(cap);
            column_line_vec.push(cell);
        }
        column_line_vec.push(divider.clone());

        let column_line = column_line_vec.into_iter().collect::<String>();
        // print rows
        let mut row_lines = String::from("");
        for display_row in display_rows {
            let mut row_line_vec = Vec::new();
            let r = display_row.raw_row;
            for (y, _) in column_display_row.raw_row.iter().enumerate() {
                row_line_vec.push(divider.clone());
                let cap = col_widths[y];
                let value: String = r.get(y).unwrap().to_string();
                let cell = value.clone().pad_to_width(cap);
                row_line_vec.push(cell);
            }
            row_line_vec.push(divider.clone());
            let row_line = row_line_vec.into_iter().collect::<String>();
            row_lines.push_str(&row_line.to_owned());
            row_lines.push_str("\n");
        }

        let mut chars = row_lines.chars();
        chars.next_back();
        let final_row_lines = chars.as_str();
        // print end stats
        let end = "Done.";

        let result_table = first_line.clone() + "\n" 
                                    + &column_line.to_owned() + "\n" 
                                    + &first_line.clone() + "\n" 
                                    + final_row_lines + "\n"
                                    + &first_line.clone() + "\n" 
                                    + end;
        return result_table;
    }
}

fn print_results(rows: Vec<Row>) {
    let results = print(rows);
    println!("{results}");
}

async fn query_async(config: Connection, query: String) {
    let host = config.host;
    let username = config.username;
    let password = config.password;
    let database = config.database;
    let params = format!(
        "host={} user={} password={} dbname={}",
        host.clone(),
        username.clone(),
        password.clone(),
        database.clone()
    );
    let connection_tuple = tokio_postgres::connect(&params.as_str(), NoTls).await;
    match connection_tuple {
        Ok((client, connection)) => {
            println!("Connected.");

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            let result = client.query(&query, &[]).await;
            match result {
                Ok(rows) => {
                    print_results(rows);
                },
                Err(e) => println!("Did not return results from query because of an error.\n{:?}", e)
            }
        },
        Err(error) => eprintln!("connection error: {}", error)
    }
}

pub fn query(config: Connection, query: String) {
    block_on(query_async(config, query))
}