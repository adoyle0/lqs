use crate::{config::Connection, display_row::{self, DisplayRow}};
use futures::executor::block_on;
use std::collections::HashMap;

use serde_json::{Map, Value};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

// const DIVIDER: &str = "|";

// pub struct DisplayRow {
//   raw_row: HashMap<String, String>,
//   value_widths: HashMap<String, usize>,
//   columns: Vec<String>,
//   // column_widths: HashMap<String, usize>,
// }

// impl DisplayRow {
//   pub fn new(row: HashMap<String, String>) -> DisplayRow {
//     let mut columns: Vec<String> = Vec::new();
//     get_column_display_line(columns, max_widths)
//     let mut value_widths: HashMap<String, usize> = HashMap::new();
//     // let mut column_widths: HashMap<String, usize> = HashMap::new();

//     row.keys().for_each(|column| {
//       columns.push(column.clone());
//       // column_widths.insert(column.clone(), column.len());
//       let row_value = row.get(column).unwrap();
//       value_widths.insert(row_value.clone(), row_value.len());
//     });

//     return DisplayRow {
//         raw_row: row,
//         value_widths,
//         columns,
//         // column_widths
//     }
//   }
// }

// fn print_results(rows: Vec<Value>) {
//   // Assuming serde Value with the structure
//   // [{key: value}] where value can be any SurrealDB data type
//   println!("Start...");
//   let display_rows = display_rows_from_rows(rows);
//   let columns = get_column_row(&display_rows);
//   let max_widths = get_max_widths(&display_rows, &columns);

//   let mut total_width: usize = max_widths.values().sum();
//   total_width = total_width + (columns.len() - 1);

//   let first_line = get_first_display_line(total_width);
//   let column_row = get_column_display_line(&columns, &max_widths);
//   let render_rows = get_data_display_lines(display_rows, columns, max_widths);

//   // Put it all together
//   let result_table = first_line.clone() + "\n"
//                               + &column_row.to_owned() + "\n"
//                               + &first_line.clone() + "\n"
//                               + &render_rows
//                               + &first_line.clone();
//   println!("{}", result_table);
// }

// fn get_data_display_lines(display_rows: Vec<DisplayRow>, columns: HashSet<String>, max_widths: HashMap<String, usize>) -> String {
//     let mut result_rows: Vec<String> = vec![];
//     for drow in display_rows {
//     let mut results_row_vec: Vec<String> = vec![DIVIDER.to_string()];
//     for column in &columns {
//       let cell_width = max_widths.get(column).unwrap().clone();
//       let value_default = String::from(""); // Make this a constant
//       let content = drow.raw_row.get(column).unwrap_or(&value_default);
//       let cell = content.pad_to_width(cell_width).clone();
//       results_row_vec.push(cell);
//       results_row_vec.push(DIVIDER.to_string());
//     }
//     let result_row = results_row_vec.into_iter().collect::<String>();
//     result_rows.push(result_row + "\n");
//       }
//     let render_rows = result_rows.into_iter().collect::<String>();
//     render_rows
// }

// fn get_column_display_line(columns: &HashSet<String>, max_widths: &HashMap<String, usize>) -> String {
//     let mut column_row_vec: Vec<String> = vec![DIVIDER.to_string()];
//     columns.iter().for_each(|column| {
//     let cell_width = max_widths.get(column).unwrap().clone();
//     let cell = column.pad_to_width(cell_width).clone();
//     column_row_vec.push(cell);
//     column_row_vec.push(DIVIDER.to_string());
//       });
//     let column_row = column_row_vec.into_iter().collect::<String>();
//     column_row
// }

// fn get_first_display_line(total_width: usize) -> String {
//     let first_line_border = vec!["-"; total_width];
//     let mut first_line_vec = vec!["+"];
//     let first_line_end = vec!["+"];
//     first_line_vec.extend_from_slice(&first_line_border);
//     first_line_vec.extend_from_slice(&first_line_end);
//     let first_line = first_line_vec.into_iter().collect::<String>();
//     first_line
// }

// fn get_column_row(display_rows: &Vec<DisplayRow>) -> HashSet<String> {
//   let mut columns: HashSet<String> = HashSet::new();
//   // Try extend() here
//   display_rows.iter().for_each(|drow| {
//     drow.columns.iter().for_each(|col| {
//       columns.insert(col.clone());
//     })
//   });

//   return columns;
// }

// fn get_max_widths(display_rows: &Vec<DisplayRow>, columns: &HashSet<String>) -> HashMap<String, usize> {
//   let mut max_widths: HashMap<String, usize> = HashMap::new();
//   // Get widths of each column into map
//   columns.iter().for_each(|column| {
//     max_widths.insert(column.clone(), column.len());
//   });

//   // Use value widths here
//   display_rows.iter().for_each(|drow| {
//     drow.columns.iter().for_each(|drow_column| {
//       let column_width = drow.value_widths.get(drow_column).unwrap();
//       if max_widths.contains_key(drow_column) {
//         if max_widths.get(drow_column).unwrap() < &column_width {
//           max_widths.insert(drow_column.clone(), column_width.clone());
//         }
//       } else {
//         max_widths.insert(drow_column.clone(), column_width.clone());
//       }
//     });
//   });
//   max_widths
// }

// fn maps_from_rows(rows: Vec<Value>) -> Vec<HashMap<String, String>> {
//     let mut display_maps: Vec<HashMap<String, String>> = Vec::new();
//     for row in &rows {
//         let object_row: &Map<String, Value> = row.as_object().unwrap();
//         let mut data_row: HashMap<String, String> = HashMap::new();

//         for column in object_row.keys() {
//             // Custom handling for id column
//             if column == "id" {
//                 let column_value = object_row.get(column).unwrap().clone();
//                 let record: Thing = serde_json::from_value(column_value).unwrap();
//                 let record_id = format!("{}:{}", record.tb, record.id);
//                 data_row.insert(column.clone(), record_id);
//             } else {
//                 match object_row.get(column) {
//                     Some(val) => {
//                         let record: String = val.to_string();
//                         data_row.insert(column.clone(), record.clone());
//                     }
//                     _ => println!("No value"),
//                 }
//             }
//         }
//         display_maps.push(data_row);
//     }
//     display_maps
// }

fn display_rows_from_rows(rows: Vec<Value>) -> Vec<DisplayRow> {
    let mut display_rows: Vec<DisplayRow> = Vec::new();
    for row in &rows {
        let object_row: &Map<String, Value> = row.as_object().unwrap();
        let mut data_row: HashMap<String, String> = HashMap::new();

        for column in object_row.keys() {
            // Custom handling for id column
            if column == "id" {
                let column_value = object_row.get(column).unwrap().clone();
                let record: Thing = serde_json::from_value(column_value).unwrap();
                let record_id = format!("{}:{}", record.tb, record.id);
                data_row.insert(column.clone(), record_id);
            } else {
                match object_row.get(column) {
                    Some(val) => {
                        let record: String = val.to_string();
                        data_row.insert(column.clone(), record.clone());
                    }
                    _ => println!("No value"),
                }
            }
        }
        display_rows.push(DisplayRow::new(data_row));
    }
    display_rows
}

async fn query_sdk(config: Connection, query: String) -> surrealdb::Result<()> {
    // Connect to the server
    let url = config.host + ":" + config.port.as_str();
    // println!("Connecting to {}...", url);
    let client = Surreal::new::<Ws>(url).await;
    match client {
        Ok(db) => {
            // println!("Cliented...");
            // Signin as a namespace, database, or root user
            db.signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await?;
            // println!("Authenticated...");

            // Select a specific namespace / database
            db.use_ns(&config.namespace)
                .use_db(&config.database)
                .await?;
            // println!("Connected.");

            let mut response = db.query(&query).await?;
            let result: Result<Vec<Value>, surrealdb::Error> = response.take(0);

            match result {
                Ok(rows) => {
                    if rows.len() == 0 {
                        println!("No results returned");
                    } else {
                        let row_map = display_rows_from_rows(rows);
                        display_row::render(row_map);
                    }
                }
                Err(e) => println!(
                    "Did not return results from query because of an error.\n{:?}",
                    e
                ),
            }
        }
        Err(error) => println!("{}", error),
    }

    Ok(())
}

async fn query_async(config: Connection, query: String) {
    let result = query_sdk(config, query).await;
    match result {
        Ok(_) => println!("Done."),
        Err(err) => println!("{}", err),
    }
}

pub fn query(config: Connection, query: String) {
    block_on(query_async(config, query))
}
