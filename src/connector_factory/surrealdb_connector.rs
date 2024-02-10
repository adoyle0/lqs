use std::collections::HashMap;

use futures::executor::block_on;
use serde_json::{Map, Value};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

use crate::{config::Connection, display_row::{self, DisplayRow}};

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
