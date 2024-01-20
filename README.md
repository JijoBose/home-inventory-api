### Home Inventory API
Built with Axum and SeaORM

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

``sea-orm-cli generate entity -u postgres://jijobose:password@localhost/loco_app -o src/database/``
