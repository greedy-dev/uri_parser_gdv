use clap::Parser;
use uri_parser::{ParseError, ParsedURI};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URI to parse
    #[arg()]
    uri: String,

    /// Verbose error display
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), ParseError>{
    let args = Args::parse();
    
    let parsed_uri = ParsedURI::parse(&args.uri)?;

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
    
    Ok(())
}
