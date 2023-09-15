# ankidb

`ankidb` gives you an opinionated API to Anki's database.

```rust
use ankidb::Database;

let db = Database::open(&"/path/to/collection.anki2")?;
let mut stmt = db.prepare_cached_raw("SELECT COUNT(*) FROM revlog")?;
let res: i64 = stmt.query_row([], |row| row.get(0))?;
assert!(res > 100);
```
