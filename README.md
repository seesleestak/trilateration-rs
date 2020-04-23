# trilateration-rs

A simple Rust implementation of trilateration. Inspired heavily by [node-trilateration](https://github.com/mberberoglu/node-trilateration).

```
USAGE:
    trilat <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>    Sets the input file to use
```

## Example

```bash
cargo run -- examples/beacons.json
```
