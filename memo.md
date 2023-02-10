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

## database

```sh
docker run --name some-postgres -v some-postgres:/var/iib/postgresql/data -e POSTGRES_PASSWORD=mysecretpassword -p 127.0.0.1:5432:5432 -d postgres 
```
