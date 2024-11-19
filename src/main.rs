use clap::{Parser, Subcommand};
use uri_parser_gdv::{ParseError, ParsedURI};

#[derive(Debug, Parser)]
#[command(
    version,
    author = "Denys Hostylo <denis@greedydev.io>",
    about = "A tool for parsing URIs and displaying individual components",
    long_about = "This CLI tool parses URIs, displaying the scheme, user info, domain, IP, port, path, and query parameters when available",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Parses provided URI
    Parse {
        #[arg(help = "The URI to parse")]
        uri: String,
    },
    /// Prints details about author
    About,
    /// Print this message or the help of the given subcommand(s)
    Help,
}

fn main() -> Result<(), ParseError> {
    let args = Cli::parse();

    match args.command {
        Commands::Parse { uri } => {
            let parsed_uri = ParsedURI::parse(&uri)?;

            print_uri_details(parsed_uri);
        }
        Commands::About => {
            print_about();
        }
        Commands::Help => {
            print_help();
        }
    }

    Ok(())
}

fn print_uri_details(parsed_uri: ParsedURI) {
    println!("Scheme: {}", parsed_uri.scheme);
    if let Some(username) = parsed_uri.username {
        println!("Username: {}", username);
    }
    if let Some(password) = parsed_uri.password {
        println!("Password: {}", password);
    }
    if let Some(domain) = parsed_uri.domain {
        println!("Domain: {}", domain);
    }
    if let Some(ip) = parsed_uri.ip {
        println!("IP: {}", ip);
    }
    if let Some(port) = parsed_uri.port {
        println!("Port: {}", port);
    }
    if let Some(path) = parsed_uri.path {
        println!("Path: {}", path);
    }
    if !parsed_uri.query_params.is_empty() {
        println!("Query params:");
        parsed_uri.query_params.iter().for_each(|param| {
            println!("  - {}: {}", param.0, param.1);
        })
    }
}

fn print_about() {
    println!("URI Parser Tool v{}", env!("CARGO_PKG_VERSION"));
    println!("Developed by: Denys Hostylo");
    println!("Description: A tool for parsing URIs and displaying components.");
}

fn print_help() {
    println!(
        "A tool for parsing URIs and displaying individual components

Usage: uri_parser_gdv <COMMAND>

Commands:
  parse  Parses provided URI
  about  Prints details about author
  help   Print this message or the help of the given subcommand(s)

Options:
  -V, --version  Print version"
    )
}
