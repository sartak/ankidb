use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

macro_rules! id_wrapper {
    ($t:ty) => {
        impl From<i64> for $t {
            #[inline]
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl From<$t> for i64 {
            #[inline]
            fn from(value: $t) -> Self {
                value.0
            }
        }

        impl ToSql for $t {
            #[inline]
            fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }

        impl FromSql for $t {
            #[inline]
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                value.as_i64().map(Into::into)
            }
        }

        impl From<$t> for sea_query::Value {
            fn from(id: $t) -> Self {
                id.0.into()
            }
        }

        impl From<&$t> for sea_query::Value {
            fn from(id: &$t) -> Self {
                id.0.into()
            }
        }

        impl sea_query::Nullable for $t {
            fn null() -> sea_query::Value {
                sea_query::Value::Int(None)
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeckId(i64);
id_wrapper!(DeckId);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotetypeId(i64);
id_wrapper!(NotetypeId);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoteId(i64);
id_wrapper!(NoteId);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardId(i64);
id_wrapper!(CardId);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RevlogId(i64);
id_wrapper!(RevlogId);
