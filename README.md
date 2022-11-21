# SDSPRPCD - Playground

Playing around with FastRPC ioctls until something happens.

## Build

```
cargo build
```

### Cross Compile

Example for shift-axolotl (aarch64) @ postmarketOS (musl)

```
cargo install -f cross

cross build --target aarch64-unknown-linux-musl
```

## License

See [LICENSE](LICENSE)
