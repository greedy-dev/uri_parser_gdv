use clap::Parser as ClapParser;
use pest_derive::Parser as PestParser;
use pest::Parser;
use pest::iterators::Pairs;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URI to parse
    #[arg()]
    uri: String,
    
    /// Verbose error display
    #[arg(short, long)]
    verbose: bool,
}

#[derive(PestParser)]
#[grammar = "grammar.pest"]
struct URIParser;

fn main() {
    let args = Args::parse();
    
    let parsed_uri_result = URIParser::parse(Rule::uri, &args.uri);
    match parsed_uri_result {
        Ok(parsed_uri) => {
            process_uri(parsed_uri);
        }
        Err(e) => {
            println!("Invalid URI found");
            if args.verbose {
                println!("{:?}", e);
            }
        }
    }
}

fn process_uri(parsed_uri: Pairs<Rule>) {
    for pair in parsed_uri {
        match pair.as_rule() {
            Rule::scheme => println!("scheme: {}", pair.as_str()),
            Rule::authority => {
                println!("authority:");
                let mut authority_inner = pair.into_inner();
                if let Some(username_pair) = authority_inner.next() {
                    println!("  - username: {}", username_pair.as_str());
                }
                if let Some(password_pair) = authority_inner.next() {
                    println!("  - password: {}", password_pair.as_str());
                }
            }
            Rule::host => {
                let host_inner = pair.into_inner().next().unwrap();
                match host_inner.as_rule() {
                    Rule::ipv4 => println!("ip: {}", host_inner.as_str()),
                    Rule::domain => println!("domain: {}", host_inner.as_str()),
                    _ => {}
                }
            }
            Rule::port => {
                let port_str = pair.as_str().trim_start_matches(':');
                println!("port: {}", port_str);
            }
            Rule::path => println!("path: {}", pair.as_str()),
            Rule::query => {
                println!("query:");
                for key_value_pair in pair.into_inner() {
                    let mut inner_pairs = key_value_pair.into_inner();
                    let key = inner_pairs.next().unwrap().as_str();
                    let value = inner_pairs.next().unwrap().as_str();
                    println!("  - {}: {}", key, value);
                }
            }
            _ => {}
        }
    }
}
