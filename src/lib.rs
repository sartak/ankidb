//! `ankidb` gives you an opinionated API to Anki's database.
//!
//! ```rust,no_run
//! use ankidb::Database;
//! use ankidb::table::*;
//! use sea_query::*;
//!
//! let db = Database::open(&"/path/to/collection.anki2")?;
//! let (mut stmt, bind) = db.prepare(
//!     Query::select()
//!         .expr(Func::count(Expr::col(Asterisk)))
//!         .from(Revlog::Table)
//! )?;
//! let res: i64 = stmt.query_row(&*bind.as_params(), |row| row.get(0))?;
//! assert!(res > 100);
//! # Ok::<(), rusqlite::Error>(())
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// https://github.com/SeaQL/sea-query/issues/706
#![allow(clippy::multiple_crate_versions)]

mod database;
pub mod model;
pub mod query;
pub mod table;

pub use database::Database;
