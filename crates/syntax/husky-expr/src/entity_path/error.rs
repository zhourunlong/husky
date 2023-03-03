use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntityPathExprError {
    #[error("original `{0}`")]
    Original(OriginalEntityPathExprError),
    #[error("derived `{0}`")]
    Derived(DerivedEntityPathExprError),
}

impl From<TokenError> for EntityPathExprError {
    fn from(value: TokenError) -> Self {
        EntityPathExprError::Derived(value.into())
    }
}

impl From<OriginalEntityPathExprError> for EntityPathExprError {
    fn from(v: OriginalEntityPathExprError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedEntityPathExprError> for EntityPathExprError {
    fn from(v: DerivedEntityPathExprError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalEntityPathExprError {
    #[error("entity tree")]
    EntityTree {
        token_idx: TokenIdx,
        error: EntityTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentifierAfterScopeResolution(TokenIdx),
}

impl OriginalError for OriginalEntityPathExprError {
    type Error = EntityPathExprError;
}

impl From<OriginalExprError> for OriginalEntityPathExprError {
    fn from(value: OriginalExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalExprError),
    #[error("token error {0}")]
    TokenError(#[from] TokenError),
}

pub type EntityPathExprResult<T> = Result<T, EntityPathExprError>;