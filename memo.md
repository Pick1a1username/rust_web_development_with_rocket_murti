# memo


## Create a New Rust Application Using Cargo

Run `cargo` command:

```
cargo new <directory name> --name <package name>
```

Add rocket info to `Cargo.toml`:

```toml
rocket = "0.5.0-rc.1"
```

Write the following lines at the top of the `src/main.rs`:

```rust
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
```