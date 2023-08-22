use crate::{lqs, config::Connection};
mod postgres_connector;

pub fn submit(connection: Connection, query: String) {
  let parsed_query = lqs::parse(query);
  let system = connection.system.clone();

  match system.as_str() {
    "postgres" => postgres_connector::query(connection, parsed_query),
    _ => println!("No system configured")
  }
}
