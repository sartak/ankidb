//! `ankidb` gives you an opinionated API to Anki's database.
//!
//! ```rust,no_run
//! use ankidb::{Database, query::{self, AnkiExt}};
//!
//! let db = Database::open(&"/path/to/collection.anki2")?;
//! let (mut stmt, bind) = db.prepare(query::revlog().count_star())?;
//! let res: i64 = stmt.query_row(&*bind.as_params(), |row| row.get(0))?;
//! assert!(res > 100);
//! # Ok::<(), rusqlite::Error>(())
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod database;
pub mod model;
pub mod query;
pub mod table;

pub use database::Database;
pub use model::{parse_fields, parse_tags};
