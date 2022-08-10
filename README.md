# Setup prisma rust client

## Rust Nightly
prisma rust client now has to run in rust nightly, to switch to rust nightly
```
rustup default nightly
```

## Prisma CLI
able to run prisma rust cli with
```
cargo prisma --your_command--
```

the config is in `.cargo/config.toml`

ref: https://github.com/Brendonovich/prisma-client-rust/blob/main/docs/01-installation.md

## Run app
in `src/main.rs`
```
cargo run
```
