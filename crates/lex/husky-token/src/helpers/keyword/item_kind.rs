use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKindKeywordGroup {
    // todo: remove mod
    Mod(ModToken),
    // `fn`
    Fn(FormFnToken),
    // `const fn`
    ConstFn(ConstToken, FormFnToken),
    // `static fn`
    StaticFn(StaticToken, FormFnToken),
    // `static const fn`
    StaticConstFn(StaticToken, ConstToken, FormFnToken),
    // `val`
    Val(ValToken),
    // `gn`
    Gn(GnToken),
    //
    GeneralDef(GeneralDefToken),
    // Type
    TypeEntity(TypeEntityToken),
    // Type
    Type(TypeToken),
    Trait(TraitToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormFnToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ConstToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            TokenData::Keyword(Keyword::Const) => Ok(Some(ConstToken { token_idx })),
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneralDefToken {
    Def(DefToken),
    Lemma(LemmaToken),
    Theorem(TheoremToken),
    Function(FunctionToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefToken {
    token_idx: TokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LemmaToken {
    token_idx: TokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheoremToken {
    token_idx: TokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeEntityToken {
    keyword: TypeEntityKeyword,
    token_idx: TokenIdx,
}
impl TypeEntityToken {
    pub fn type_kind(self) -> TypeKind {
        // MOM
        match self.keyword {
            TypeEntityKeyword::Extern => TypeKind::Extern,
            TypeEntityKeyword::Struct => TypeKind::Struct,
            TypeEntityKeyword::Enum => TypeKind::Enum,
            TypeEntityKeyword::Record => TypeKind::Record,
            TypeEntityKeyword::Structure => TypeKind::Structure,
            TypeEntityKeyword::Inductive => TypeKind::Inductive,
        }
    }

    pub fn keyword(self) -> TypeEntityKeyword {
        self.keyword
    }

    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValToken {
    token_idx: TokenIdx,
}

impl ValToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GnToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EntityKindKeywordGroup
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        let kw = match token {
            TokenData::Keyword(kw) => kw,
            TokenData::Error(error) => Err(error)?,
            _ => return Ok(None),
        };
        match kw {
            Keyword::Fugitive(kw) => match kw {
                FugitiveKeyword::Def => todo!(),
                FugitiveKeyword::Fn => {
                    Ok(Some(EntityKindKeywordGroup::Fn(FormFnToken { token_idx })))
                }
                FugitiveKeyword::Theorem => Ok(Some(EntityKindKeywordGroup::GeneralDef(
                    GeneralDefToken::Theorem(TheoremToken { token_idx }),
                ))),
                FugitiveKeyword::Lemma => todo!(),
                FugitiveKeyword::Proposition => todo!(),
                FugitiveKeyword::Type => {
                    Ok(Some(EntityKindKeywordGroup::Type(TypeToken { token_idx })))
                }
                FugitiveKeyword::Val => {
                    Ok(Some(EntityKindKeywordGroup::Val(ValToken { token_idx })))
                }
                FugitiveKeyword::Gn => Ok(Some(EntityKindKeywordGroup::Gn(GnToken { token_idx }))),
                FugitiveKeyword::Constexpr => todo!(),
            },
            Keyword::TypeEntity(keyword) => {
                Ok(Some(EntityKindKeywordGroup::TypeEntity(TypeEntityToken {
                    keyword,
                    token_idx,
                })))
            }
            Keyword::Stmt(_) => todo!(),
            Keyword::Mod => Ok(Some(EntityKindKeywordGroup::Mod(ModToken { token_idx }))),
            Keyword::Trait => Ok(Some(EntityKindKeywordGroup::Trait(TraitToken {
                token_idx,
            }))),
            Keyword::Const => todo!(),
            Keyword::Static => match token_stream.peek() {
                Some(TokenData::Keyword(Keyword::Fugitive(FugitiveKeyword::Fn))) => {
                    token_stream.next();
                    Ok(Some(EntityKindKeywordGroup::StaticFn(
                        StaticToken { token_idx },
                        FormFnToken {
                            token_idx: token_idx + 1,
                        },
                    )))
                }
                Some(TokenData::Keyword(Keyword::Const)) => todo!(),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
