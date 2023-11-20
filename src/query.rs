#![allow(clippy::similar_names)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::return_self_not_must_use)]

use crate::{model::*, table::*};
use sea_query::*;

pub enum FieldMatcher {
    Any,
    Equals(String),
    Prefix(String),
    Suffix(String),
    Contains(String),
}

#[must_use]
pub fn col() -> SelectStatement {
    Query::select().from(Col::Table).take()
}

#[must_use]
pub fn notes() -> SelectStatement {
    Query::select().from(Notes::Table).take()
}

#[must_use]
pub fn cards() -> SelectStatement {
    Query::select().from(Cards::Table).take()
}

#[must_use]
pub fn revlog() -> SelectStatement {
    Query::select().from(Revlog::Table).take()
}

#[must_use]
pub fn graves() -> SelectStatement {
    Query::select().from(Graves::Table).take()
}

#[must_use]
pub fn deck_config() -> SelectStatement {
    Query::select().from(DeckConfig::Table).take()
}

#[must_use]
pub fn config() -> SelectStatement {
    Query::select().from(Config::Table).take()
}

#[must_use]
pub fn fields() -> SelectStatement {
    Query::select().from(Fields::Table).take()
}

#[must_use]
pub fn templates() -> SelectStatement {
    Query::select().from(Templates::Table).take()
}

#[must_use]
pub fn notetypes() -> SelectStatement {
    Query::select().from(Notetypes::Table).take()
}

#[must_use]
pub fn decks() -> SelectStatement {
    Query::select().from(Decks::Table).take()
}

#[must_use]
pub fn tags() -> SelectStatement {
    Query::select().from(Tags::Table).take()
}

pub trait AnkiExt {
    fn get_nid(self) -> Self;
    fn get_cid(self) -> Self;
    fn get_mid(self) -> Self;
    fn get_did(self) -> Self;

    fn get_fields(self) -> Self;
    fn get_tags(self) -> Self;
    fn get_queue(self) -> Self;

    fn where_mid(self, mid: NotetypeId) -> Self;
    fn where_mids(self, mids: &[NotetypeId]) -> Self;
    fn where_cards_ord(self, ord: i64) -> Self;
    fn where_cards_type(self, r#type: i64) -> Self;
    fn where_cards_queue(self, queues: &[i64]) -> Self;
    fn where_templates_name(self, name: &str) -> Self;
    fn where_suspended(self, suspended: bool) -> Self;
    fn where_fields_like(self, pattern: &str) -> Self;
    fn where_fields_match(self, fields: &[FieldMatcher]) -> Self;

    fn not_did_mid(self, did: DeckId, mid: NotetypeId) -> Self;

    fn join_notes_cards(self) -> Self;
    fn join_cards_notes(self) -> Self;
    fn join_notetypes(self) -> Self;
    fn join_templates(self) -> Self;

    fn count_star(self) -> Self;
}

impl AnkiExt for &mut SelectStatement {
    fn get_nid(self) -> Self {
        self.column((Notes::Table, Cards::Id))
    }

    fn get_cid(self) -> Self {
        self.column((Cards::Table, Cards::Id))
    }

    fn get_mid(self) -> Self {
        self.column((Notes::Table, Notes::Mid))
    }

    fn get_did(self) -> Self {
        self.column((Cards::Table, Cards::Did))
    }

    fn get_fields(self) -> Self {
        self.column((Notes::Table, Notes::Flds))
    }

    fn get_tags(self) -> Self {
        self.column((Notes::Table, Notes::Tags))
    }

    fn get_queue(self) -> Self {
        self.column((Cards::Table, Cards::Queue))
    }

    fn where_mid(self, mid: NotetypeId) -> Self {
        self.and_where(Expr::col((Notes::Table, Notes::Mid)).eq(mid))
    }

    fn where_mids(self, mids: &[NotetypeId]) -> Self {
        self.and_where(Expr::col((Notes::Table, Notes::Mid)).is_in(mids))
    }

    fn where_cards_ord(self, ord: i64) -> Self {
        self.and_where(Expr::col((Cards::Table, Cards::Ord)).eq(ord))
    }

    fn where_cards_type(self, r#type: i64) -> Self {
        self.and_where(Expr::col((Cards::Table, Cards::Type)).eq(r#type))
    }

    fn where_cards_queue(self, queues: &[i64]) -> Self {
        self.and_where(Expr::col((Cards::Table, Cards::Queue)).is_in(queues.iter().copied()))
    }

    fn where_templates_name(self, name: &str) -> Self {
        self.and_where(Expr::col((Templates::Table, Templates::Name)).eq(name))
    }

    fn where_suspended(self, suspended: bool) -> Self {
        self.conditions(
            suspended,
            |q| {
                q.and_where(Expr::col((Cards::Table, Cards::Queue)).lt(0));
            },
            |q| {
                q.and_where(Expr::col((Cards::Table, Cards::Queue)).gte(0));
            },
        )
    }

    fn where_fields_like(self, pattern: &str) -> Self {
        self.and_where(Expr::col((Notes::Table, Notes::Flds)).like(pattern))
    }

    /// NOTE: The provided `fields` list must have the same length as the note's fields
    /// definition; otherwise, will give inconsistent results.
    fn where_fields_match(self, fields: &[FieldMatcher]) -> Self {
        let spec = fields
            .into_iter()
            .map(|f| match f {
                FieldMatcher::Any => String::from("%"),
                FieldMatcher::Equals(s) => s.clone(),
                FieldMatcher::Prefix(s) => format!("{s}%"),
                FieldMatcher::Suffix(s) => format!("%{s}"),
                FieldMatcher::Contains(s) => format!("%{s}%"),
            })
            .collect::<Vec<_>>()
            .join("\x1F");

        self.where_fields_like(&spec)
    }

    fn not_did_mid(self, did: DeckId, mid: NotetypeId) -> Self {
        self.cond_where(
            Cond::all()
                .not()
                .add(Expr::col((Cards::Table, Cards::Did)).eq(did))
                .add(Expr::col((Notes::Table, Notes::Mid)).eq(mid)),
        )
    }

    fn join_notes_cards(self) -> Self {
        self.left_join(
            Cards::Table,
            Expr::col((Cards::Table, Cards::Nid)).equals((Notes::Table, Notes::Id)),
        )
    }

    fn join_cards_notes(self) -> Self {
        self.left_join(
            Notes::Table,
            Expr::col((Cards::Table, Cards::Nid)).equals((Notes::Table, Notes::Id)),
        )
    }

    fn join_notetypes(self) -> Self {
        self.left_join(
            Notetypes::Table,
            Expr::col((Notes::Table, Notes::Mid)).equals((Notetypes::Table, Notetypes::Id)),
        )
    }

    fn join_templates(self) -> Self {
        self.left_join(
            Templates::Table,
            Expr::col((Notetypes::Table, Notetypes::Id))
                .equals((Templates::Table, Templates::Ntid)),
        )
    }

    fn count_star(self) -> Self {
        self.expr(Func::count(Expr::col(Asterisk)))
    }
}
