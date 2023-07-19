mod derive_decr;

pub use self::derive_decr::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum DecrSignatureTemplate {
    Derive(DeriveDecrDeclarativeSignatureTemplate),
}