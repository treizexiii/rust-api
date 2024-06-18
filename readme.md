# webapi

## Development

#### Server side:

```bash
cargo install cargo-watch
cargo install --locked cargo-watch # on windows
```

```bash
cargo watch -q -c -w src/ -x run
```

#### Client side:

```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```


## Build

### Docker:
```bash
docker build -f .\Dockerfile -t rustapi:1.0 . 
```

```bash
docker run -p 8080:8080 --name rust-api rustapi:1.0
```