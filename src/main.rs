use clap::{Parser, Subcommand};
use uri_parser_gdv::{ParseError, ParsedURI};

#[derive(Debug, Parser)]
#[command(
    version,
    author = "Denys Hostylo <denis@greedydev.io>",
    about = "A tool for parsing URIs and displaying individual components",
    long_about = "This CLI tool parses URIs, displaying the scheme, user info, domain, IP, port, path, and query parameters when available",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Parse {
        #[arg(help = "The URI to parse")]
        uri: String,
    },
    About,
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