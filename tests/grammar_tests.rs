use pest::Parser;
use uri_parser_gdv::{Rule, URIParser};

mod tests {
    use super::*;
    
    #[test]
    fn valid_scheme() {
        let res = URIParser::parse(Rule::scheme, "https");
        assert!(res.is_ok());
    }
    
    #[test]
    fn invalid_scheme() {
        let res = URIParser::parse(Rule::scheme, "1:http");
        
        assert!(res.is_err());
    }
    
    #[test]
    fn valid_host() {
        let res = URIParser::parse(Rule::host, "greedydev.io");
        assert!(res.is_ok());
    }
    
    #[test]
    fn invalid_host() {
        let res = URIParser::parse(Rule::host, "123");
        assert!(res.is_err());
    }
    
    #[test]
    fn valid_query() {
        let res = URIParser::parse(Rule::query, "?one=1&two=2");
        assert!(res.is_ok());
    }
    
    #[test]
    fn invalid_query() {
        let res = URIParser::parse(Rule::query, "!one=1&two");
        assert!(res.is_err());
    }
    
    #[test]
    fn valid_port() {
        let res = URIParser::parse(Rule::port, ":8080");
        assert!(res.is_ok());
    }

    #[test]
    fn valid_uri() {
        let res = URIParser::parse(Rule::uri, "wss://hello:world@greedydev.io:8080/hello/world?msg=one&lvl=2");
        assert!(res.is_ok());
    }

    #[test]
    fn invalid_uri() {
        let res = URIParser::parse(Rule::uri, "/greedydev.io?msg=one&lvl=2");
        assert!(res.is_err());
    }
}
