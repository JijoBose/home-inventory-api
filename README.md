### Home Inventory API
App to manage your inventories

#### Tech Stack

**Backend**
- Rust 🦀
- Axum
- SeaORM

**Frontend**
- HTMX

<img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" height="100px" />
<img src="https://raw.githubusercontent.com/bigskysoftware/htmx/master/www/static/img/htmx_logo.1.png" height="100px"/>


## Development

#### Compile the current package
- ``cargo build``

#### Run the App

- ``cargo run``


## How to's

**Create new table**

``sea-orm-cli migrate generate create_room``

**Migrate**

- ``sea-orm-cli migrate``

**Update Entity**

``sea-orm-cli generate entity -u postgres://user:password@localhost/home_inventory -o entity/src``
