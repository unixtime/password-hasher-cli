use std::env;
use std::io::{self, stdin, Write};
use std::path::Path;

use argon2::{self, Config};
use bcrypt::{DEFAULT_COST, hash};
use clap::{Arg, ArgAction, Command};
use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use dotenv;
use rpassword::read_password;
use urlencoding::encode;

// Struct to hold database configuration
struct DatabaseConfig {
    host: String,
    db: String,
    user: String,
    pass: String,
    port: u16,
}

// Enum to handle different types of database connections
enum DatabaseConnection {
    Mysql(MysqlConnection),
    Postgres(PgConnection),
    Sqlite(SqliteConnection),
}

impl DatabaseConnection {
    // Establishes a connection to the specified database
    fn establish(db_type: &str) -> Result<Self, String> {
        match db_type {
            "mysql" => {
                let config = DatabaseConfig {
                    host: env::var("MYSQL_HOST").unwrap(),
                    db: env::var("MYSQL_DB").unwrap(),
                    user: env::var("MYSQL_USER").unwrap(),
                    pass: encode(&env::var("MYSQL_PASS").unwrap()).parse().unwrap(),
                    port: env::var("MYSQL_PORT").unwrap().parse::<u16>().unwrap(),
                };
                let database_url = format!("mysql://{}:{}@{}:{}/{}",
                                           config.user, config.pass, config.host, config.port, config.db);
                MysqlConnection::establish(&database_url)
                    .map(DatabaseConnection::Mysql)
                    .map_err(|e| e.to_string())
            },
            "postgres" => {
                let config = DatabaseConfig {
                    host: env::var("POSTGRES_HOST").unwrap(),
                    db: env::var("POSTGRES_DB").unwrap(),
                    user: env::var("POSTGRES_USER").unwrap(),
                    pass: encode(&env::var("POSTGRES_PASS").unwrap()).parse().unwrap(),
                    port: env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap(),
                };
                let database_url = format!("postgres://{}:{}@{}:{}/{}",
                                           config.user, config.pass, config.host, config.port, config.db);
                PgConnection::establish(&database_url)
                    .map(DatabaseConnection::Postgres)
                    .map_err(|e| e.to_string())
            },
            "sqlite" => {
                let database_url = env::var("SQLITE_DATABASE_URL").unwrap();
                SqliteConnection::establish(&database_url)
                    .map(DatabaseConnection::Sqlite)
                    .map_err(|e| e.to_string())
            },
            _ => Err(format!("Unsupported database type: {}", db_type)),
        }
    }

    // Executes a query on the established database connection
    fn execute_query(&mut self, query: &str) -> Result<usize, String> {
        match self {
            DatabaseConnection::Mysql(conn) => sql_query(query).execute(conn),
            DatabaseConnection::Postgres(conn) => sql_query(query).execute(conn),
            DatabaseConnection::Sqlite(conn) => sql_query(query).execute(conn),
        }.map_err(|e| e.to_string())
    }
}

// Utility function to prompt the user and get input, ensuring non-empty input
fn prompt_non_empty(message: &str) -> String {
    loop {
        print!("{}", message);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
        println!("Input cannot be empty. Please try again.");
    }
}


fn main() {
    dotenv::from_path(Path::new(".env")).ok();

    let matches = Command::new("Password Hasher CLI")
        .version("1.0")
        .author("Hassan El-Masri <hassan@unixtime.com>")
        .about("Handles passwords for databases or standalone hashing")
        .arg(Arg::new("pass")
            .long("pass")
            .help("Hashes a password without database interaction")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("method")
            .long("method")
            .help("Specifies the hashing method: argon2 or bcrypt")
            .default_value("bcrypt")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(String)))
        .get_matches();

    let method = matches.get_one::<String>("method").unwrap().as_str();

    // Check for the "pass" argument
    if let Some(password) = matches.get_one::<String>("pass") {
        let hashed_password = match method {
            "argon2" => {
                let salt = b"randomsalt"; // In real applications, use a secure random salt
                let config = Config::default();
                argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
            },
            _ => hash(password, DEFAULT_COST).expect("Failed to hash password"), // Bcrypt as default
        };
        println!("Hashed Password: {}", hashed_password);
        return;
    }

    println!("Password Hasher CLI");
    let db_type = prompt_non_empty("Enter database type (mysql, postgres, sqlite): ");
    let table_name = prompt_non_empty("Enter table name: ");
    let user_identifier = prompt_non_empty("Enter user ID or username to change password: ");

    println!("Please enter a password to hash (input will be hidden): ");
    let password = read_password().unwrap();
    println!("Please confirm your password: ");
    let confirm_password = read_password().unwrap();

    if password != confirm_password {
        println!("Passwords do not match. Please try again.");
        return;
    }
    if password.is_empty() {
        println!("No password entered.");
        return;
    }

    let hashed_password = match method {
        "argon2" => {
            let salt = b"randomsalt";
            let config = Config::default();
            argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
        },
        _ => hash(&password, DEFAULT_COST).expect("Failed to hash password"),
    };

    let mut connection = DatabaseConnection::establish(&db_type).expect("Failed to establish database connection");
    let query = if user_identifier.parse::<i32>().is_ok() {
        format!("UPDATE {} SET password = '{}' WHERE user_id = '{}'", table_name, hashed_password, user_identifier)
    } else {
        format!("UPDATE {} SET password = '{}' WHERE username = '{}'", table_name, hashed_password, user_identifier)
    };
    connection.execute_query(&query).expect("Failed to execute query");
    println!("Password updated successfully.");
}