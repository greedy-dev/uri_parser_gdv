use thiserror::Error;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct URIParser;

pub struct ParsedURI {
    /// URI scheme
    pub scheme: String,
    /// Authority -- username
    pub username: Option<String>,
    /// Authority -- password
    pub password: Option<String>,
    /// Domain component
    pub domain: Option<String>,
    /// IP component
    pub ip: Option<String>,
    pub port: Option<String>,
    pub path: Option<String>,
    pub query_params: Vec<(String, String)>,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Can't parse URI")]
    ParsingFailed,
    
    #[error("Unexpected parsing rule encountered: {0}")]
    UnexpectedParsingRule(String),
}

impl ParsedURI {
    pub fn parse(uri: &str) -> Result<Self, ParseError> {
        let pairs = URIParser::parse(Rule::uri, uri)
            .map_err(|_| ParseError::ParsingFailed)?;
        
        let mut scheme = String::new();
        let mut username = None;
        let mut password = None;
        let mut domain = None;
        let mut ip = None;
        let mut port = None;
        let mut path = None;
        let mut query_params: Vec<(String, String)> = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::scheme => scheme = pair.as_str().to_string(),
                Rule::authority => {
                    let mut authority_inner = pair.into_inner();
                    if let Some(username_pair) = authority_inner.next() {
                        username = Some(username_pair.as_str().to_string())
                    }
                    if let Some(password_pair) = authority_inner.next() {
                        password = Some(password_pair.as_str().to_string());
                    }
                }
                Rule::host => {
                    let host_inner = pair.into_inner().next().unwrap();
                    match host_inner.as_rule() {
                        Rule::ipv4 => ip = Some(host_inner.as_str().to_string()),
                        Rule::domain => domain = Some(host_inner.as_str().to_string()),
                        _ => {}
                    }
                }
                Rule::port => {
                    let port_str = pair.as_str().trim_start_matches(':');
                    port = Some(port_str.to_string())
                },
                Rule::path => path = Some(pair.as_str().to_string()),
                Rule::query => {
                    for key_value_pair in pair.into_inner() {
                        let mut inner_pairs = key_value_pair.into_inner();
                        let key = inner_pairs.next().unwrap().as_str();
                        let value = inner_pairs.next().unwrap().as_str();
                        query_params.push((key.to_string(), value.to_string()));
                    }
                }
                rule => return Err(ParseError::UnexpectedParsingRule(format!("{:?}", rule))),
            }
        }

        Ok(Self {
            scheme,
            username,
            password,
            domain,
            ip,
            port,
            path,
            query_params
        })
    }
}