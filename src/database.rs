use crate::model::{DeckId, NotetypeId};
use rusqlite::{params, Connection, Result};
use sea_query::SqliteQueryBuilder;
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};
use std::path::Path;
use unicase::UniCase;

pub struct Database {
    connection: Connection,
}

impl Database {
    /// Opens a connection to an Anki database.
    ///
    /// ```rust,no_run
    /// use ankidb::Database;
    ///
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail for a number of reasons: the database does not exist or is unreadable, changing
    /// configuration settings on the database handle fails, etc.
    pub fn open<P: AsRef<Path>>(path: &P) -> Result<Self> {
        // Connection::open, but without the CREATE flag
        let db = Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
                | rusqlite::OpenFlags::SQLITE_OPEN_URI
                | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        // This is the same config that Anki uses, though without exclusive locking
        // since this library is meant to coexist with other tools.
        // https://github.com/ankitects/anki/blob/30ae9f7c5408420c8f347073a9e5e62756a6d7cb/rslib/src/storage/sqlite.rs#L53-L64

        db.busy_timeout(std::time::Duration::from_secs(0))?;

        // db.pragma_update(None, "locking_mode", &"exclusive")?;
        db.pragma_update(None, "page_size", 4096)?;
        db.pragma_update(None, "cache_size", -40 * 1024)?;
        db.pragma_update(None, "legacy_file_format", false)?;
        db.pragma_update(None, "journal_mode", "wal")?;

        db.set_prepared_statement_cache_capacity(50);

        db.create_collation("unicase", |s1, s2| UniCase::new(s1).cmp(&UniCase::new(s2)))?;

        Ok(Self { connection: db })
    }

    /// Prepares a seaquery statement to run against the db.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// use sea_query::*;
    /// use ankidb::table::Revlog;
    /// # let db = Database::open(&"/path/to/collection.anki2")?;
    /// let (mut stmt, bind) = db.prepare(
    ///     Query::select()
    ///         .expr(Func::count(Expr::col(Asterisk)))
    ///         .from(Revlog::Table)
    /// )?;
    /// let res: i64 = stmt.query_row(&*bind.as_params(), |row| row.get(0))?;
    /// assert!(res > 100);
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if there's a syntax error, or if the database becomes unavailable.
    pub fn prepare<T: RusqliteBinder>(
        &self,
        query: &T,
    ) -> Result<(rusqlite::Statement<'_>, RusqliteValues)> {
        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
        self.connection.prepare(&sql).map(|s| (s, values))
    }

    /// Prepares a seaquery statement to run against the db.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// use sea_query::*;
    /// use ankidb::table::Revlog;
    /// # let db = Database::open(&"/path/to/collection.anki2")?;
    /// let (mut stmt, bind) = db.prepare_cached(
    ///     Query::select()
    ///         .expr(Func::count(Expr::col(Asterisk)))
    ///         .from(Revlog::Table)
    /// )?;
    /// let res: i64 = stmt.query_row(&*bind.as_params(), |row| row.get(0))?;
    /// assert!(res > 100);
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if there's a syntax error, or if the database becomes unavailable.
    pub fn prepare_cached<T: RusqliteBinder>(
        &self,
        query: &T,
    ) -> Result<(rusqlite::CachedStatement<'_>, RusqliteValues)> {
        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
        self.connection.prepare_cached(&sql).map(|s| (s, values))
    }

    /// Prepares a SQL-string statement to run against the db.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// let mut stmt = db.prepare_raw("SELECT COUNT(*) FROM revlog")?;
    /// let res: i64 = stmt.query_row([], |row| row.get(0))?;
    /// assert!(res > 100);
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if there's a syntax error, or if the database becomes unavailable.
    pub fn prepare_raw(&self, sql: &str) -> Result<rusqlite::Statement<'_>> {
        self.connection.prepare(sql)
    }

    /// Prepares a cached SQL-string statement to run against the db.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// let mut stmt = db.prepare_cached_raw("SELECT COUNT(*) FROM revlog")?;
    /// let res: i64 = stmt.query_row([], |row| row.get(0))?;
    /// assert!(res > 100);
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if there's a syntax error, or if the database becomes unavailable.
    pub fn prepare_cached_raw(&self, sql: &str) -> Result<rusqlite::CachedStatement<'_>> {
        self.connection.prepare_cached(sql)
    }

    /// Gets the id of a deck by its name.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// let id = db.id_for_deck("General")?;
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if the provided name does not match a deck, or if the
    /// database becomes unavailable.
    pub fn id_for_deck(&self, name: &str) -> Result<DeckId> {
        let mut stmt = self.prepare_raw("SELECT id FROM decks WHERE name=?")?;
        stmt.query_row(params![name], |row| row.get(0))
    }

    /// Gets the id of a notetype by its name.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// let id = db.id_for_notetype("Basic")?;
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if the provided name does not match a notetype, or if the
    /// database becomes unavailable.
    pub fn id_for_notetype(&self, name: &str) -> Result<NotetypeId> {
        let mut stmt = self.prepare_raw("SELECT id FROM notetypes WHERE name=?")?;
        stmt.query_row(params![name], |row| row.get(0))
    }

    /// Gets the names of each field for the given notetype id.
    ///
    /// ```rust,no_run
    /// # use ankidb::Database;
    /// let db = Database::open(&"/path/to/collection.anki2")?;
    /// let id = db.id_for_notetype("Basic")?;
    /// let fields = db.fields_for_notetype(id)?;
    /// assert_eq!(fields[0], "Front");
    /// assert_eq!(fields[1], "Back");
    /// # Ok::<(), rusqlite::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if the provided id does not match a notetype, or if the
    /// database becomes unavailable.
    pub fn fields_for_notetype(&self, id: NotetypeId) -> Result<Vec<String>> {
        let mut stmt = self.prepare_raw("SELECT name FROM fields WHERE ntid=? ORDER BY ord ASC")?;
        let res = stmt.query_map(params![id], |row| row.get(0))?;
        res.collect()
    }
}
