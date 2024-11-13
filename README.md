# uri_parser
URI Parser is a pest-based parser that breaks URIs into structured components

The following components are supported:
- Scheme
- Authority: username and password
- Domain / IPv4
- Port
- Path (segmented by /)
- Query params
- Subdomain: Optional subdomain that appears before the main domain.
## Usage
#### Command
```
uri_parser [OPTIONS] <URI>
```
#### Args and opts
```
  <URI>  URI to parse
  -v, --verbose  Verbose error display
  -h, --help     Print help
  -V, --version  Print version
```

## Example
```
wss://user:pass@greedydev.io:1234/hello/world?msg=message&type=2
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