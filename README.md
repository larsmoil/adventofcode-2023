# My solutions for Advent of Code 2023

## Run tests
```shell
cargo test
```

If you want the optimized version:
```shell
RUST_MIN_STACK=8388608 cargo test --release
```

## Run
```shell
cargo run
```

If you want the optimized version:
```shell
cargo run --release
```


## Development
I usually use `cargo watch` with the following arguments:
```shell
cargo watch \
  -x 'fmt' \
  -x 'test' \
  -x 'clippy -- -W clippy::pedantic -W clippy::cargo' \
  -x run \
  -x 'run --release'
```
