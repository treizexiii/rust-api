# webapi

## Development

Server side:

```bash
cargo install cargo-watch
cargo install --locked cargo-watch # on windows
```

```bash
cargo watch -q -c -w src/ -x run
```

Client side:

```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
