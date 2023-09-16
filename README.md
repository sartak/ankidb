# ankidb

`ankidb` gives you an opinionated API to Anki's database.

```rust
use ankidb::Database;
use ankidb::table::*;
use sea_query::*;

let db = Database::open(&"/path/to/collection.anki2")?;
let (mut stmt, bind) = db.prepare(
    Query::select()
        .expr(Func::count(Expr::col(Asterisk)))
        .from(Revlog::Table)
)?;
let res: i64 = stmt.query_row(&*bind.as_params(), |row| row.get(0))?;
assert!(res > 100);
```
