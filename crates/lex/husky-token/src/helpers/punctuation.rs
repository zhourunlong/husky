use super::*;
use husky_opn_syntax::{BinaryOpr, Bracket};

// punctuation in general

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct PunctuationToken {
    punc: Punctuation,
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PunctuationToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

// specific punctuation

fn parse_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(TokenIdx) -> T,
) -> TokenResult<Option<T>>
where
    Context: TokenStreamParser<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            Token::Punctuation(punc) if punc == target => Ok(Some(f(token_idx))),
            Token::Error(error) => Err(error),
            Token::Label(_)
            | Token::Punctuation(_)
            | Token::Ident(_)
            | Token::WordOpr(_)
            | Token::Literal(_)
            | Token::Keyword(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

// colon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ColonToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ColonToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::COLON, ColonToken)
    }
}

#[test]
fn colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ",").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// semicolon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]

pub struct SemiColonToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for SemiColonToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::SEMICOLON, SemiColonToken)
    }
}

#[test]
fn semicolon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<SemiColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ";").unwrap().is_some());
    assert!(t(&db, ":").unwrap().is_none());
    assert!(t(&db, ",").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// comma

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct CommaToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for CommaToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::COMMA, CommaToken)
    }
}

#[test]
fn comma_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CommaToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ",").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// eq `=`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct EqToken(TokenIdx);

impl EqToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EqToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::EQ, EqToken)
    }
}

#[test]
fn eq_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<EqToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "=").unwrap().is_some());
    assert!(t(&db, "=:").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LeftParenthesisToken(TokenIdx);

impl LeftParenthesisToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LeftParenthesisToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LPAR, LeftParenthesisToken)
    }
}

#[test]
fn left_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "(").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RightParenthesisToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for RightParenthesisToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::RPAR, RightParenthesisToken)
    }
}

#[test]
fn right_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ")").unwrap().is_some());
    assert!(t(&db, "(").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LeftBoxBracketToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LeftBoxBracketToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LBOX, LeftBoxBracketToken)
    }
}

#[test]
fn left_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "[").unwrap().is_some());
    assert!(t(&db, "]").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RightBoxBracketToken(TokenIdx);

impl RightBoxBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for RightBoxBracketToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::RBOX, RightBoxBracketToken)
    }
}

#[test]
fn right_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "]").unwrap().is_some());
    assert!(t(&db, "[").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LeftCurlyBraceToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LeftCurlyBraceToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LCURL, LeftCurlyBraceToken)
    }
}

#[test]
fn left_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "{").unwrap().is_some());
    assert!(t(&db, "}").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RightCurlyBraceToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for RightCurlyBraceToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::RCURL, RightCurlyBraceToken)
    }
}

#[test]
fn right_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "}").unwrap().is_some());
    assert!(t(&db, "{").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LeftAngleBracketOrLessThanToken(TokenIdx);

impl LeftAngleBracketOrLessThanToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LeftAngleBracketOrLessThanToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LA_OR_LT, LeftAngleBracketOrLessThanToken)
    }
}

#[test]
fn left_angle_or_less_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftAngleBracketOrLessThanToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "<").unwrap().is_some());
    assert!(t(&db, "::<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// colon colon left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ColonColonLeftAngleBracketToken(TokenIdx);

impl ColonColonLeftAngleBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ColonColonLeftAngleBracketToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(
            ctx,
            Punctuation::COLON_COLON_LA,
            ColonColonLeftAngleBracketToken,
        )
    }
}

#[test]
fn colon_colon_left_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonColonLeftAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "::<").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RightAngleBracketToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for RightAngleBracketToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::RA_OR_GT, RightAngleBracketToken)
    }
}

#[test]
fn right_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, ">").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// `/>`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct EmptyHtmlKetToken(TokenIdx);

impl EmptyHtmlKetToken {
    pub fn token_idx(self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EmptyHtmlKetToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::EMPTY_HTML_KET, EmptyHtmlKetToken)
    }
}

#[test]
fn empty_html_ket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<EmptyHtmlKetToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "/>").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// vertical

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct VerticalToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for VerticalToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::VERTICAL, VerticalToken)
    }
}

#[test]
fn vertical_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<VerticalToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "|").unwrap().is_some());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// at

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct AtToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for AtToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::AT, AtToken)
    }
}

#[test]
fn at_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<AtToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "@").unwrap().is_some());
    assert!(t(&db, "|").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// dotdot

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct DotDotToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for DotDotToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::DOT_DOT, DotDotToken)
    }
}

#[test]
fn dotdot_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<DotDotToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "..").unwrap().is_some());
    assert!(t(&db, "...").unwrap().is_none());
    assert!(t(&db, "@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// dotdotdot `...`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct DotDotDotToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for DotDotDotToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::DOT_DOT_DOT, DotDotDotToken)
    }
}

#[test]
fn dot_dot_dot_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<DotDotDotToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "...").unwrap().is_some());
    assert!(t(&db, "..").unwrap().is_none());
    assert!(t(&db, "@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `:` at the end of line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub enum EolToken {
    Colon(EolColonToken),
    Semicolon(EolSemicolonToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct EolColonToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct EolSemicolonToken {
    token_idx: TokenIdx,
}

impl EolToken {
    pub fn token_idx(&self) -> TokenIdx {
        match self {
            EolToken::Colon(token) => token.token_idx,
            EolToken::Semicolon(token) => token.token_idx,
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::COLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Colon(EolColonToken { token_idx }))),
                },
                Token::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Semicolon(EolSemicolonToken { token_idx }))),
                },
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn eol_colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<EolToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `::`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ScopeResolutionToken(TokenIdx);

impl ScopeResolutionToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ScopeResolutionToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::COLON_COLON) => {
                    Ok(Some(ScopeResolutionToken(token_idx)))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn scope_resolution_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ScopeResolutionToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "::").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `*`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct StarToken(TokenIdx);

impl StarToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for StarToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::STAR) => Ok(Some(StarToken(token_idx))),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn star_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<StarToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "*").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct CurryToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for CurryToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::LIGHT_ARROW) => Ok(Some(CurryToken(token_idx))),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn curry_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CurryToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "->").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `!!`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct OwnedToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for OwnedToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::DOUBLE_EXCLAMATION) => {
                    Ok(Some(OwnedToken(token_idx)))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn double_exclamation_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<OwnedToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "!!").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `:=`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ColonEqToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ColonEqToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::COLON_EQ, ColonEqToken)
    }
}

#[test]
fn colon_eq_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonEqToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":=").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LightArrowToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LightArrowToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LIGHT_ARROW, LightArrowToken)
    }
}

#[test]
fn light_arrow_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<LightArrowToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "->").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `=>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct HeavyArrowToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for HeavyArrowToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::HEAVY_ARROW, HeavyArrowToken)
    }
}

#[test]
fn heavy_arrow_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<HeavyArrowToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "=>").unwrap().is_some());
    assert!(t(&db, "->").unwrap().is_none());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}