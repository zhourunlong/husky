use crate::*;
use husky_dec_term::term::DecSymbolicVariableTypeErrorKind;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_syn_decl::error::SynDeclError;
use thiserror::Error;

pub type DeclarativeTypeResult<T> = Result<T, DeclarativeTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum DeclarativeTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalDeclarativeTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedDeclarativeTypeError),
}

impl From<DecSymbolicVariableTypeErrorKind> for DeclarativeTypeError {
    fn from(e: DecSymbolicVariableTypeErrorKind) -> Self {
        DerivedDeclarativeTypeError::SymbolType(e).into()
    }
}

impl From<&SynDeclError> for DeclarativeTypeError {
    fn from(_value: &SynDeclError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum OriginalDeclarativeTypeError {
    #[error("declarative_term error")]
    DecTerm(#[from] DecTermError),
    #[error("EnumHasNoConstructor")]
    EnumNoConstructor,
    #[error("InductiveTypeHasNoConstructor")]
    InductiveTypeHasNoConstructor,
    #[error("todo")]
    Todo,
    #[error("todo")]
    ExternTypeHasNoConstructor,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum DerivedDeclarativeTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("type constructor declaration error")]
    TypeConstructorDeclError,
    #[error("type ontology declaration error")]
    TypeOntologyDeclError { path: TypePath },
    #[error("trait declaration error")]
    TraitDeclError,
    #[error("form declaration error")]
    FormDeclError,
    #[error("type path field declaration error")]
    TypePathFieldDeclError,
    #[error("type path application field declaration error")]
    TypePathApplicationFieldDeclError,
    #[error("type path method declaration error")]
    TypePathMethodFnDeclError,
    #[error("type path application method declaration error")]
    TypePathApplicationMethodFnDeclError,
    #[error("TypeItemNotFound")]
    TypeItemNotFound,
    #[error("SymbolType")]
    SymbolType(DecSymbolicVariableTypeErrorKind),
}
