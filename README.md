# uri_parser_gdv

Crates.io: https://crates.io/crates/uri_parser_gdv/

---
URI Parser is a pest-based parser that breaks URIs into structured components

### Parsing Logic
![Parsing logic](https://i.imgur.com/3aygL2j.png)

The following components are supported:
- Scheme
- Authority: username and password
- Domain / IPv4
- Port
- Path (segmented by /)
- Query params
- Subdomain: Optional subdomain that appears before the main domain.

## Usage
#### Parsing URIs
```
uri_parser_gdv parse <URI>
```
#### Subcommands
```
  about          Print author's details
```
#### Args and opts
```
  -h, --help     Print help
  -V, --version  Print version
```

## Example
```
uri_parser_gdv parse wss://user:pass@greedydev.io:1234/hello/world?msg=message&type=2
```
#### Output:
```
scheme: wss
authority:
  - username: user
  - password: pass
domain: greedydev.io
port: 1234
path: /hello/world
query:
  - msg: message
  - type: 2
```