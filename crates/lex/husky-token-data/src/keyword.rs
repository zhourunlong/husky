mod ambiguous;
mod connection;
mod end;
mod form;
mod modifier;
mod pronoun;
mod stmt;
mod ty;

pub use self::ambiguous::*;
pub use self::connection::*;
pub use self::end::*;
pub use self::form::*;
pub use self::modifier::*;
pub use self::pronoun::*;
pub use self::stmt::*;
pub use self::ty::*;

use crate::*;

pub use modifier::*;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = TokenDataDb)]
#[enum_class::from_variants]
pub enum Keyword {
    Fugitive(FugitiveKeyword),
    TypeEntity(TypeEntityKeyword),
    Stmt(StmtKeyword),
    Modifier(ModifierKeyword),
    Pronoun(PronounKeyword),
    Use,
    Mod,
    Impl,
    Trait,
    Connection(ConnectionKeyword),
    End(EndKeyword),
    Pub,
    Const,
    Static,
    Async,
    Sorry,
    Todo,
    Unreachable,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl Keyword {
    pub const fn code(&self) -> &'static str {
        match self {
            Keyword::Fugitive(keyword) => keyword.code(),
            Keyword::TypeEntity(keyword) => keyword.code(),
            Keyword::Stmt(keyword) => keyword.code(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Modifier(keyword) => keyword.code(),
            Keyword::Trait => "trait",
            Keyword::Impl => "impl",
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => "pub",
            Keyword::Const => "const",
            Keyword::Static => "static",
            Keyword::Async => "async",
            Keyword::Sorry => "sorry",
            Keyword::Todo => "todo",
            Keyword::Unreachable => "unreachable",
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}