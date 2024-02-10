use std::collections::HashMap;

use futures::executor::block_on;
use postgres::{NoTls, Row};
use tokio_postgres::Column;
use chrono::Utc;
use uuid::Uuid;

use crate::config::Connection;
use crate::display_row;

/// The postgres-crate does not provide a default mapping to fallback to String for all
/// types: row.get is generic and without a type assignment the FromSql-Trait cannot be inferred.
/// This function matches over the current column-type and does a manual conversion
/// TODO Maybe try Serde instead
fn parse_column_to_string(row: &postgres::Row, column: &Column) -> String {
    let column_type = column.type_().name();
    let column_name = column.name();
    // see https://docs.rs/sqlx/0.4.0-beta.1/sqlx/postgres/types/index.html
    let value = match column_type {
        "bool" => {
            let v: Option<bool> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "varchar" | "char(n)" | "text" | "name" => {
            let v: Option<String> = row.get(column_name);
            v
        }
        "int2" | "smallserial" | "smallint" => {
            let v: Option<i16> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "int" | "int4" | "serial" => {
            let v: Option<i32> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "int8" | "bigserial" | "bigint" => {
            let v: Option<i64> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "float4" | "real" => {
            let v: Option<f32> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "float8" | "double precision" => {
            let v: Option<f64> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "timestamp" | "timestamptz" => {
            // with-chrono feature is needed for this
            let v: Option<chrono::DateTime<Utc>> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        "uuid" => {
            let v: Option<Uuid> = row.get(column_name);
            v.map(|v| v.to_string())
        }
        &_ => Some("CANNOT PARSE".to_string()),
    };
    value.unwrap_or("".to_string())
}

/// Convert Postgres Rows to Hashmap for DisplayRow conversion
fn rows_to_map(rows: Vec<Row>) -> Vec<HashMap<String, String>> {
    // Convert rows to hash map
    let mut maps: Vec<HashMap<String, String>> = Vec::new();
    rows.iter().for_each(|row| {
        let mut map: HashMap<String, String> = HashMap::new();
        row.columns().into_iter().for_each(|col| {
            let val = parse_column_to_string(row, col);
            map.insert(col.name().to_string(), val);
        });
        maps.push(map);
    });
    

    return maps;
}

/// Async query Postgres with given connection configuration and query
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

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            let result = client.query(&query, &[]).await;
            match result {
                Ok(rows) => {
                    let maps = rows_to_map(rows);
                    let display_rows = display_row::display_rows_from_maps(maps);
                    display_row::render(display_rows)
                },
                Err(e) => println!("Did not return results from query because of an error.\n{:?}", e)
            }
        },
        Err(error) => eprintln!("connection error: {}", error)
    }
}

/// Entry point for querying using Postgres connector
pub fn query(config: Connection, query: String) {
    block_on(query_async(config, query))
}