# ankidb

`ankidb` gives you an opinionated API to Anki's database.

```rust
use ankidb::Database;

let db = Database::open(&"/path/to/collection.anki2")?;
```
