![FIle Router workflow](https://github.com/unixtime/password-hasher-cli/actions/workflows/rust.yml/badge.svg)

# Password Hasher CLI Tool

This CLI tool provides password hashing capabilities and password reset functionality for a user recorded in a database.
It allows for securely hashing passwords using `bcrypt` or `argon2` and managing them within various databases, including MySQL, PostgreSQL, and SQLite. 
It can also operate in a standalone mode, where it simply hashes a password using a specified method without interacting with a database.

## Requirements

- Rust (latest stable release)
- Cargo (Rust's package manager)

## Supported Databases

- PostgreSQL
- MySQL/MariaDB
- SQLite

## Setup Instructions

1. **Clone the Repository:**
   Please install Rust and Cargo, then clone this repository to your local machine.

```bash
git clone https://github.com/unixtime/password-hasher-cli.git
cd password-hasher-cli
```

## Environment Configuration

Rename `example.env` to `.env` and update the file to match your database credentials:

```dotenv
# PostgreSQL Database Configuration
POSTGRES_HOST='hostname'
POSTGRES_DB='database_name'
POSTGRES_USER='username'
POSTGRES_PASS='password'
POSTGRES_PORT=5432

# MySQL Database Configuration
MYSQL_HOST='hostname'
MYSQL_DB='database_name'
MYSQL_USER='username'
MYSQL_PASS='password'
MYSQL_PORT=3306

# SQLite Database Configuration
SQLITE_DATABASE_URL=sqlite:///path/to/your/database.db
```

## Build the Application

Compile the application with Cargo:

```bash
cargo build --release
```

This command compiles the application and outputs the executable in target/release/.

## Run the Application

To run the application, use the following command:

```bash
cargo run
```

## Deployment

Copy the compiled executable to a directory in your PATH for easier execution:

```bash
cp target/release/password_hasher ~/bin/  # or any other directory in your PATH
```

## Usage

Usage
This application can be used in two modes: Standalone password hashing or database password management.

### Standalone Password Hashing

To hash a password without interacting with a database:

```bash
password_hasher_cli --method argon2 --pass "SomePass"
```

Available methods for hashing are `argon2` and `bcrypt`. 
By default, `bcrypt` is used if no method is specified.

## Running the Application

Follow the on-screen prompts to select your database type, enter database credentials, and manage user passwords.

## Test the Application SQLite

### Creating the SQLite Users Table

Here’s an example of how you might set up a user table in SQLite to use with this tool:

```bash
sqlite3 users.db
```

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

INSERT INTO users (username, password) VALUES ('tester', 'hashed_password');
```

### Running the Tool with SQLite and SQLite Users Table Example

```bash
password_hasher_cli --method argon2 # or bcrypt

Password Hasher CLI
Enter database type (mysql, postgres, sqlite): sqlite
Enter table name: users
Enter the user ID or username to change password: 1
Please enter a password to hash (input will be hidden): 

Please confirm your password: 

Password updated successfully.
```

Additional Information
This tool uses Diesel for ORM operations and bcrypt for password hashing. For more information on these dependencies, visit:

* Diesel: https://diesel.rs/
* bcrypt: https://docs.rs/bcrypt/latest/bcrypt/
* dotenv: https://docs.rs/dotenv/0.15.0/dotenv/
* rust-argon2: https://docs.rs/rust-argon2/latest/argon2/index.html
* clap: https://docs.rs/clap/4.5.4/clap/index.html
* rand: https://docs.rs/rand/latest/rand/
* rpassword: https://docs.rs/rpassword/latest/rpassword/
* urlencoding: https://docs.rs/urlencoding/2.1.3/urlencoding/

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
