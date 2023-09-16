use sea_query::Iden;

/// The `col` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Col {
    // create table col
    Table,
    // id integer primary key
    Id,
    // crt integer not null
    Crt,
    // mod integer not null
    Mod,
    // scm integer not null
    Scm,
    // ver integer not null
    Ver,
    // dty integer not null
    Dty,
    // usn integer not null
    Usn,
    // ls integer not null
    Ls,
    // conf text not null
    Conf,
    // models text not null
    Models,
    // decks text not null
    Decks,
    // dconf text not null
    Dconf,
    // tags text not null
    Tags,
}

/// The `notes` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Notes {
    // create table notes
    Table,
    // id integer primary key
    Id,
    // guid text not null
    Guid,
    // mid integer not null
    Mid,
    // mod integer not null
    Mod,
    // usn integer not null
    Usn,
    // tags text not null
    Tags,
    // flds text not null
    Flds,
    // sfld integer not null
    Sfld,
    // csum integer not null
    Csum,
    // flags integer not null
    Flags,
    // data text not null
    Data,
}

/// The `cards` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Cards {
    // create table cards
    Table,
    // id integer primary key
    Id,
    // nid integer not null
    Nid,
    // did integer not null
    Did,
    // ord integer not null
    Ord,
    // mod integer not null
    Mod,
    // usn integer not null
    Usn,
    // type integer not null
    Type,
    // queue integer not null
    Queue,
    // due integer not null
    Due,
    // ivl integer not null
    Ivl,
    // factor integer not null
    Factor,
    // reps integer not null
    Reps,
    // lapses integer not null
    Lapses,
    // left integer not null
    Left,
    // odue integer not null
    Odue,
    // odid integer not null
    Odid,
    // flags integer not null
    Flags,
    // data text not null
    Data,
}

/// The `revlog` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Revlog {
    // create table revlog
    Table,
    // id integer primary key
    Id,
    // cid integer not null
    Cid,
    // usn integer not null
    Usn,
    // ease integer not null
    Ease,
    // ivl integer not null
    Ivl,
    // lastivl integer not null
    Lastivl,
    // factor integer not null
    Factor,
    // time integer not null
    Time,
    // type integer not null
    Type,
}

/// The `graves` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Graves {
    // create table graves
    Table,
    // usn integer not null
    Usn,
    // oid integer not null
    Oid,
    // type integer not null
    Type,
}

/// The `deck_config` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum DeckConfig {
    // create table deck_config
    Table,
    // id integer primary key not null
    Id,
    // name text not null collate unicase
    Name,
    // mtime_secs integer not null
    MtimeSecs,
    // usn integer not null
    Usn,
    // config blob not null
    Config,
}

/// The `config` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Config {
    // create table config
    Table,
    // key text not null primary key
    Key,
    // usn integer not null
    Usn,
    // mtime_secs integer not null
    MtimeSecs,
    // val blob not null
    Val,
}

/// The `fields` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Fields {
    // create table fields
    Table,
    // ntid integer not null
    Ntid,
    // ord integer not null
    Ord,
    // name text not null collate unicase
    Name,
    // config blob not null
    Config,
}

/// The `templates` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Templates {
    // create table templates
    Table,
    // ntid integer not null
    Ntid,
    // ord integer not null
    Ord,
    // name text not null collate unicase
    Name,
    // mtime_secs integer not null
    MtimeSecs,
    // usn integer not null
    Usn,
    // config blob not null
    Config,
}

/// The `notetypes` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Notetypes {
    // create table notetypes
    Table,
    // id integer not null primary key
    Id,
    // name text not null collate unicase
    Name,
    // mtime_secs integer not null
    MtimeSecs,
    // usn integer not null
    Usn,
    // config blob not null
    Config,
}

/// The `decks` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Decks {
    // create table decks
    Table,
    // id integer primary key not null
    Id,
    // name text not null collate unicase
    Name,
    // mtime_secs integer not null
    MtimeSecs,
    // usn integer not null
    Usn,
    // common blob not null
    Common,
    // kind blob not null
    Kind,
}

/// The `tags` table
#[derive(Debug, Copy, Clone, Iden)]
pub enum Tags {
    // create table tags
    Table,
    // tag text not null primary key collate unicase
    Tag,
    // usn integer not null
    Usn,
    // collapsed boolean not null
    Collapsed,
    // config blob null
    Config,
}
