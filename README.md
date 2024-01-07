### Home Inventory API
Built with actix

#### Development

Setup the database and migration

``diesel setup``

**Create new table**

``diesel migration generate create_homes``

Run Migration

``diesel migration run``

Undo Migration

``diesel migration redo``

Run Cargo

``cargo run``
