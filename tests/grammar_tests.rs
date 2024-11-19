use pest::Parser;
use uri_parser_gdv::{Rule, URIParser};

mod tests {
    use super::*;

    #[test]
    fn test_scheme() -> Result<(), Box<dyn std::error::Error>> {
        let scheme = "pest-rs.scheme2+";
        let parsed_scheme = URIParser::parse(Rule::scheme, scheme)?;

        assert_eq!(parsed_scheme.as_str(), scheme);

        Ok(())
    }

    #[test]
    fn test_authority_full() -> Result<(), Box<dyn std::error::Error>> {
        let username = "testuser";
        let password = "testpass";
        let authority = format!("{}:{}", username, password);
        let mut parsed_authority = URIParser::parse(Rule::authority, authority.as_str())?
            .next()
            .unwrap()
            .into_inner();

        // Username and password
        assert_eq!(parsed_authority.len(), 2);

        assert_eq!(parsed_authority.next().unwrap().as_str(), username);
        assert_eq!(parsed_authority.next().unwrap().as_str(), password);

        Ok(())
    }

    #[test]
    fn test_authority_only_username() -> Result<(), Box<dyn std::error::Error>> {
        let username = "testuser";
        let mut parsed_authority = URIParser::parse(Rule::authority, username)?
            .next()
            .unwrap()
            .into_inner();

        // Only username
        assert_eq!(parsed_authority.len(), 1);
        assert_eq!(parsed_authority.next().unwrap().as_str(), username);

        Ok(())
    }

    #[test]
    fn test_host() -> Result<(), Box<dyn std::error::Error>> {
        let host = "test.greedydev.io";
        let parsed_host = URIParser::parse(Rule::host, host)?;

        assert_eq!(parsed_host.as_str(), host);

        Ok(())
    }

    #[test]
    fn test_query() -> Result<(), Box<dyn std::error::Error>> {
        let query = "?one=1&two";
        let mut parsed_query = URIParser::parse(Rule::query, query)?
            .next()
            .unwrap()
            .into_inner();

        let mut first_component = parsed_query.next().unwrap().into_inner();

        assert_eq!(first_component.next().unwrap().as_str(), "one");
        assert_eq!(first_component.next().unwrap().as_str(), "1");

        let mut second_component = parsed_query.next().unwrap().into_inner();

        assert_eq!(second_component.next().unwrap().as_str(), "two");

        Ok(())
    }

    #[test]
    fn test_port() -> Result<(), Box<dyn std::error::Error>> {
        let port = "8080";
        let parsed_port = URIParser::parse(Rule::port, port)?;

        assert_eq!(parsed_port.as_str(), port);

        Ok(())
    }

    #[test]
    fn test_uri() -> Result<(), Box<dyn std::error::Error>> {
        let scheme = "wss";
        let username = "user";
        let password = "pass";
        let domain = "greedydev.io";
        let port = "8080";
        let path = "/hello/worlds";
        let query = "?msg=one&lvl=2";
        let uri = format!("{scheme}://{username}:{password}@{domain}:{port}{path}{query}");

        let mut parsed_uri = URIParser::parse(Rule::uri, uri.as_str())?;

        assert_eq!(parsed_uri.next().unwrap().as_str(), scheme);

        let mut authority = parsed_uri.next().unwrap().into_inner();

        assert_eq!(authority.next().unwrap().as_str(), username);
        assert_eq!(authority.next().unwrap().as_str(), password);

        assert_eq!(parsed_uri.next().unwrap().as_str(), domain);
        assert_eq!(parsed_uri.next().unwrap().as_str(), port);
        assert_eq!(parsed_uri.next().unwrap().as_str(), path);

        let mut parsed_query = parsed_uri.next().unwrap().into_inner();

        assert_eq!(parsed_query.len(), 2);

        let mut first_component = parsed_query.next().unwrap().into_inner();

        assert_eq!(first_component.next().unwrap().as_str(), "msg");
        assert_eq!(first_component.next().unwrap().as_str(), "one");

        let mut second_component = parsed_query.next().unwrap().into_inner();

        assert_eq!(second_component.next().unwrap().as_str(), "lvl");
        assert_eq!(second_component.next().unwrap().as_str(), "2");

        Ok(())
    }
}
