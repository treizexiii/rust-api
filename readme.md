# webapi

## Development

### Tools

```bash
cargo install cargo-watch
cargo install --locked cargo-watch # on windows
```

### Server side

```bash
cargo watch -q -c -w src/ -w .cargo/ -x run
```

### Client side

```bash
cargo watch -q -c -w examples/ -x "test -q quick_dev -- --nocapture"
```

### Start db

```bash
docker run -d --rm --name postgres -p 5434:5432 -e POSTGRES_PASSWORD=welcome postgres:15
```

start pg terminal

```bash
docker exec -it -u postgres postgres psql
```

## Build

### Docker

```bash
docker build -f .\Dockerfile -t rustapi:1.0 . 
```

```bash
docker run -p 8080:8080 --name rust-api rustapi:1.0
```

## Unit test

```bash
cargo watch -q -c -x "test -- --nocapture"

cargo watch -q -c -x "test {MOD_NAME} -- --nocapture"
```
