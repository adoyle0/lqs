use std::collections::HashMap;
use clap::{Parser, Subcommand};
use config::Connection;

mod lqs;
mod config;
mod connector_factory;
mod display_row;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Connection from config file
    #[arg(short, long)]
    connection: Option<String>,

    /// Query to run against connection
    #[arg(short, long)]
    query: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes lqs cli
    Init
}

/// Start mode to run query after query without rerunning program.
fn start_continuous_querying(connection_name: String) {
    let mut run = true;

    let connection = Connection::from_config(connection_name);
    println!("Connection configured for {}...", &connection.system);

    while run {
        let mut line = String::new();
        println!("Type query: (press enter to submit)");
        std::io::stdin().read_line(&mut line).unwrap();  
        if line == String::from("exit\n") {
            run = false;
        } else {
            // Load config outside of while loop?
            connector_factory::submit(connection.clone(), line);
        }
    }
}

/// Run database query
fn run_query(connection_name: String, query: String) {
    let connection = Connection::from_config(connection_name);
    println!("Connection configured for {}...", &connection.system);
    connector_factory::submit(connection, query);
}

/// Validate database connection that is passed in
fn validate_connection(connection: Option<String>) -> Result<String, &'static str> {
    match connection {
        Some(connection_value) => {
            let connection_name = connection_value;
            let config_loaded: Result<HashMap<String, HashMap<String, Option<String>>>, String> = config::load();
            if config_loaded.is_ok() && config_loaded.unwrap().get(&connection_name) != None {
                // Validate connection is the right struct
                if Connection::from_config(connection_name.to_string()).name.is_empty() {
                    return Err("Connection not found");
                }
                return Ok(connection_name.clone());
            } else {
                return Err("Connection not found");
            }
        }
        None => {
            return Err("--connection not set");
        } 
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            config::create_config();
        }
        None => {
            match validate_connection(cli.connection) {
                Ok(connection) => {
                    match cli.query {
                        Some(input_query) => {
                            run_query(connection, input_query);
                        }
                        None => {
                            start_continuous_querying(connection);
                        }
                    }
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }
}