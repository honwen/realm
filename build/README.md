## Build Standalone Binaries

### Build with `cross`

- Install [`cross`](https://github.com/rust-embedded/cross)

```bash
cargo install cross
```

- Build with cross

```bash
cross build --target x86_64-unknown-linux-musl
```

### Predefined build routines

- `build-release`: Build binaries with `cross` and packages outputs into `release` folder
