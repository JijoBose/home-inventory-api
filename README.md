### Home Inventory API
Built with actix

## Development

#### Compile the current package
- ``cargo build``

#### Setup the database and migration

- ``diesel setup``
- ``diesel migration run``

#### Run the App

- ``cargo run``



## How to's

**Create new table**

``diesel migration generate create_homes``

**Run Migration**

``diesel migration run``

**Undo Migration**

``diesel migration redo``

sea-orm-cli generate entity -u protocol://user:password@localhost/database_name -o entity/src
sea-orm-cli migrate
