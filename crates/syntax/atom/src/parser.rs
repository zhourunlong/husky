mod impl_basic;
mod impl_func_head;
mod impl_inner_ops;
mod impl_lambda_head;
mod impl_scope;
mod utils;

use core::slice::Iter;

use check_utils::should;
use entity_route::{EntityKind, EntityRoute, EntityRouteKind, GenericArgument};
use file::FilePtr;
use print_utils::p;
use text::TextRange;
use token::{Special, Token, TokenKind};
use vm::{BinaryOpr, PureBinaryOpr};
use word::CustomIdentifier;

use super::{stack::AtomStack, symbol_proxy::SymbolProxy, *};
use crate::*;

use utils::*;

#[derive(Debug, Clone)]
pub(crate) struct Stream<'a> {
    pub(crate) iter: Iter<'a, Token>,
    pub(crate) range: TextRange,
}

impl<'a> Stream<'a> {
    pub(crate) fn next(&mut self) -> Option<&'a Token> {
        if let Some(token) = self.iter.next() {
            self.range.end = token.text_end();
            Some(token)
        } else {
            None
        }
    }

    pub(crate) fn pop_range(&mut self) -> TextRange {
        should!(self.range.start != self.range.end);
        let range = self.range.clone();
        self.range = ((self.range.end)..(self.range.end)).into();
        range
    }
}

impl<'a> From<&'a [Token]> for Stream<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter(),
            range: tokens.into(),
        }
    }
}

pub struct AtomLRParser<'a> {
    scope_proxy: SymbolProxy<'a>,
    pub(crate) stream: Stream<'a>,
    stack: AtomStack,
}

impl<'a> AtomLRParser<'a> {
    pub fn new(scope_proxy: SymbolProxy<'a>, tokens: &'a [Token]) -> Self {
        Self {
            scope_proxy,
            stream: tokens.into(),
            stack: AtomStack::new(),
        }
    }

    pub fn parse_all(mut self) -> AtomResult<Vec<Atom>> {
        loop {
            if self.stack.is_concave() {
                if let Some(kind) = try_get!(self, symbol?) {
                    self.push(kind)?;
                }
            }

            if let Some(token) = self.stream.next() {
                match token.kind {
                    TokenKind::Keyword(keyword) => {
                        err!("keyword should be put at start", token.text_range())?
                    }
                    TokenKind::Special(special) => match special {
                        Special::DoubleColon => {
                            err!("unexpected double colon, maybe the identifier before is not recognized as scope", token.text_range())?
                        }
                        Special::Colon => {
                            if let Some(_) = self.stream.next() {
                                todo!()
                            } else {
                                break;
                            }
                        }
                        Special::DoubleVertical => self.stack.push(Atom::new(
                            token.text_range(),
                            if !self.stack.is_concave() {
                                BinaryOpr::Pure(PureBinaryOpr::BitOr).into()
                            } else {
                                AtomKind::LambdaHead(Vec::new())
                            },
                        ))?,
                        Special::Vertical => {
                            let lambda_head = self.lambda_head()?;
                            self.stack.push(Atom::new(
                                (token.text_start()..self.stream.range.end).into(),
                                AtomKind::LambdaHead(lambda_head),
                            ))?;
                        }
                        Special::Ambersand => self.stack.push(Atom::new(
                            token.text_range(),
                            if self.stack.is_concave() {
                                PrefixOpr::Shared.into()
                            } else {
                                BinaryOpr::Pure(PureBinaryOpr::BitAnd).into()
                            },
                        ))?,
                        Special::LPar => self.stack.start_list(Bracket::Par, token.text_range()),
                        Special::LBox => self.stack.start_list(Bracket::Box, token.text_range()),
                        Special::LCurl => self.stack.start_list(Bracket::Curl, token.text_range()),
                        Special::RPar => {
                            if next_matches!(self, Special::LightArrow) {
                                let output = get!(self, ty?);
                                self.stack.make_func_type(
                                    self.scope_proxy,
                                    output,
                                    self.stream.pop_range(),
                                )?;
                            } else {
                                self.stack.end_list_or_make_type(
                                    Bracket::Par,
                                    ListEndAttr::None,
                                    token.text_range(),
                                    self.scope_proxy,
                                )?
                            }
                        }
                        Special::RBox => self.stack.end_list_or_make_type(
                            Bracket::Box,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        Special::RCurl => self.stack.end_list_or_make_type(
                            Bracket::Curl,
                            ListEndAttr::None,
                            token.text_range(),
                            self.scope_proxy,
                        )?,
                        Special::SubOrMinus => {
                            if self.stack.is_convex() {
                                self.stack
                                    .push(Atom::new(token.text_range(), BinaryOpr::Pure(PureBinaryOpr::Sub).into()))?
                            } else {
                                self.stack
                                    .push(Atom::new(token.text_range(), PrefixOpr::Minus.into()))?
                            }
                        }
                        Special::MemberAccess =>  {
                            let field_ident_token = self.stream.next().ok_or(error!("expect identifier after `.`", token.text_range()))?;
                            self.stack
                        .push(Atom::new(token.text_range(), SuffixOpr::MembAccess(field_ident_token.ranged_custom_ident().unwrap()).into()))?},
                        _ => self.stack.push(token.into())?,
                    },
                    _ => self.stack.push(token.into())?,
                }
            } else {
                break;
            }
        }
        Ok(self.stack.into())
    }
}

pub fn parse_ty(scope_proxy: SymbolProxy, tokens: &[Token]) -> AtomResult<EntityRoutePtr> {
    let result = AtomLRParser::new(scope_proxy, tokens.into()).parse_all()?;
    if result.len() == 0 {
        panic!()
    }
    if result.len() > 1 {
        p!(result);
        err!("too many atoms", result[1..].into())?
    } else {
        match result[0].kind {
            AtomKind::EntityRoute {
                route: scope,
                kind: EntityKind::Type(_),
                ..
            } => Ok(scope),
            AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
            _ => err!(
                format!("expect type, but get `{:?}` instead", result[0]),
                (&result).into()
            )?,
        }
    }
}

pub fn parse_entity(scope_proxy: SymbolProxy, tokens: &[Token]) -> AtomResult<EntityRoutePtr> {
    let result = AtomLRParser::new(scope_proxy, tokens.into()).parse_all()?;
    if result.len() == 0 {
        panic!()
    }
    if result.len() > 1 {
        p!(result);
        err!("too many atoms", result[1..].into())?
    } else {
        match result[0].kind {
            AtomKind::EntityRoute { route: scope, .. } => Ok(scope),
            AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
            _ => err!(
                format!("expect type, but get `{:?}` instead", result[0]),
                (&result).into()
            )?,
        }
    }
}
