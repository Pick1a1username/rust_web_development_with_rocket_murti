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

```
export DATABASE_URL=postgres://postgres:mysecretpassword@127.0.0.1:5432/postgres
```

```
sqlx database create
sqlx migrate add create_users
sqlx migrate run
```

### insert records

```sql
INSERT INTO users (uuid, username, email, password_hash, description, status)
VALUES
('00000000-0000-0000-0000-000000000000', 'testuser0', 'testuser0@test.com', 'no_password', 'testuser0 description', 1),
('00000000-0000-0000-0000-000000000001', 'testuser1', 'testuser1@test.com', 'no_password', 'testuser1 description', 1);

INSERT INTO posts (uuid, user_uuid, post_type, content)
VALUES
('11111111-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 0, 'Lorem ipsum'),
('11111111-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', 1, '/assets/443822918_97d2ae0e60.jpg'),
('11111111-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 2, '/assets/clock.mp4');
```