pub use self::matcher::*;

use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn calc_ritchie_arguments_ty(
        &mut self,
        expr_idx: SynExprIdx,
        ritchie_parameters: &[FlyRitchieParameter],
        ritchie_arguments: impl Iterator<Item = SynCallListItem> + Clone,
    ) -> SemaExprDataResult<RitchieArgumentes> {
        RitchieArgumenter::new(ritchie_parameters, ritchie_arguments.clone(), self)
            .match_all()
            .map_err(|match_error| {
                OriginalSemaExprDataError::RitchieParameterArgumentMismatch {
                    match_error,
                    ritchie_arguments: ritchie_arguments
                        .map(|ritchie_argument| {
                            self.build_sema_expr(
                                ritchie_argument.argument_expr_idx(),
                                ExpectAnyDerived,
                            )
                        })
                        .collect(),
                }
                .into()
            })
    }
}

mod matcher {
    use super::*;
    use thiserror::Error;

    #[salsa::derive_debug_with_db]
    #[derive(Debug, Error, PartialEq, Eq)]
    pub enum RitchieArgumentError {
        #[error("unexpected argument")]
        UnexpectedArgument,
        #[error("missing argument")]
        MissingArgument,
    }
    pub type RitchieArgumentResult<T> = Result<T, RitchieArgumentError>;

    #[salsa::derive_debug_with_db]
    #[derive(Debug, PartialEq, Eq)]
    pub enum SemaRitchieArgument {
        Simple(FlyRitchieSimpleParameter, SemaSimpleArgument),
        Variadic(
            FlyRitchieVariadicParameter,
            // use vec to save enum size
            Vec<SemaVariadicCallListItem>,
        ),
        Keyed(FlyRitchieKeyedParameter, Option<SemaKeyedCallListItem>),
    }

    pub type RitchieArgumentes = SmallVec<[SemaRitchieArgument; 4]>;

    pub(super) struct RitchieArgumenter<'a, 'b, Arguments: Iterator<Item = SynCallListItem>> {
        ritchie_parameters: &'b [FlyRitchieParameter],
        ritchie_call_items: std::iter::Peekable<Arguments>,
        ritchie_matches: RitchieArgumentes,
        engine: &'b mut SemaExprBuilder<'a>,
    }

    impl<'a, 'b, Arguments: Iterator<Item = SynCallListItem>> RitchieArgumenter<'a, 'b, Arguments> {
        pub(super) fn new(
            ritchie_parameters: &'b [FlyRitchieParameter],
            ritchie_arguments: Arguments,
            engine: &'b mut SemaExprBuilder<'a>,
        ) -> RitchieArgumenter<'a, 'b, impl Iterator<Item = SynCallListItem>> {
            RitchieArgumenter {
                ritchie_parameters,
                ritchie_call_items: ritchie_arguments.peekable(),
                ritchie_matches: Default::default(),
                engine,
            }
        }

        pub(super) fn match_all(mut self) -> RitchieArgumentResult<RitchieArgumentes> {
            for ritchie_parameter in self.ritchie_parameters {
                self.match_step(*ritchie_parameter)?
            }
            match self.ritchie_call_items.next() {
                Some(_) => Err(RitchieArgumentError::UnexpectedArgument)?,
                None => Ok(self.ritchie_matches),
            }
        }

        fn match_step(&mut self, param: FlyRitchieParameter) -> RitchieArgumentResult<()> {
            match param {
                FlyRitchieParameter::Simple(param) => match self.ritchie_call_items.next() {
                    Some(item) => match item {
                        SynCallListItem::SimpleOrVariadic(item) => {
                            let (argument_sema_expr_idx, coersion) =
                                self.engine.build_sema_expr_with_outcome(
                                    item.argument_expr_idx(),
                                    ExpectCoersion::new(param.contract(), param.ty()),
                                );
                            let item = SemaSimpleArgument::new(
                                argument_sema_expr_idx,
                                coersion,
                                item.separator(),
                            );
                            Ok(self
                                .ritchie_matches
                                .push(SemaRitchieArgument::Simple(param, item)))
                        }
                        SynCallListItem::Keyed(_) => todo!(),
                    },
                    None => Err(RitchieArgumentError::MissingArgument)?,
                },
                FlyRitchieParameter::Variadic(param) => {
                    let mut items = vec![];
                    while let Some(SynCallListItem::SimpleOrVariadic(item)) = self
                        .ritchie_call_items
                        .next_if(|item| matches!(item, SynCallListItem::SimpleOrVariadic(_)))
                    {
                        let (argument_sema_expr_idx, coersion_outcome) =
                            self.engine.build_sema_expr_with_outcome(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            );
                        items.push(SemaVariadicCallListItem::new(
                            argument_sema_expr_idx,
                            coersion_outcome,
                            item.separator(),
                        ));
                        match item.separator() {
                            CallListSeparator::None | CallListSeparator::Comma(_) => (),
                            CallListSeparator::Semicolon(_) => break,
                        }
                    }
                    Ok(self
                        .ritchie_matches
                        .push(SemaRitchieArgument::Variadic(param, items)))
                }
                FlyRitchieParameter::Keyed(param) => match param.has_default() {
                    true => {
                        let item =  if let Some(SynCallListItem::Keyed(item)) = self.ritchie_call_items
                            .next_if(|arg|
                                matches!(arg, SynCallListItem::Keyed(item) if item.key() == param.key())
                            ) {
                            Some(self.engine.build_sema_keyed_call_list_item(item, param))
                        } else {
                            None
                        };
                        Ok(self
                            .ritchie_matches
                            .push(SemaRitchieArgument::Keyed(param, item)))
                    }
                    false => todo!(),
                },
            }
        }
    }
}
