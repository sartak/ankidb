//! `ankidb` gives you an opinionated API to Anki's database.
//!
//! ```rust,no_run
//! use ankidb::Database;
//!
//! let db = Database::open(&"/path/to/collection.anki2")?;
//! # Ok::<(), rusqlite::Error>(())
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod database;

pub use database::Database;
