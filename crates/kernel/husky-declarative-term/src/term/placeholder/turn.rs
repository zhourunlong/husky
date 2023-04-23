use husky_entity_path::TypePath;

use crate::utils::DeclarativeTermFamily;

use super::*;

impl DeclarativeTerm {
    /// the only way to create new variable
    /// this is not cached because
    /// - it's not called frequently
    /// - it's not computationally expensively
    pub fn turn_symbol_into_variable(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
    ) -> (Self, Option<DeclarativeTermVariable>) {
        let Some(idx) = self.new_variable_idx(db, symbol) else {
            return (self, None);
        };
        let variable = DeclarativeTermVariable::new(db, symbol.ty(db), idx);
        (
            self.substitute_symbol_with_variable(db, symbol, variable),
            Some(variable),
        )
    }

    /// returns the variable idx if turning this symbol into variable
    /// returns None if symbol is not present
    #[inline(always)]
    fn new_variable_idx(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
    ) -> Option<u8> {
        self.new_variable_idx_aux(db, symbol, symbol.ty_family(db))
    }

    /// with symbol_ty_family already fetched from db
    #[inline(always)]
    fn new_variable_idx_aux(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        symbol_ty_family: DeclarativeTermFamily,
    ) -> Option<u8> {
        self.contains_symbol(db, symbol)
            .then(|| self.new_variable_idx_if_symbol_is_present(db, symbol, symbol_ty_family))
    }

    // todo: needs thorough testing
    fn new_variable_idx_if_symbol_is_present(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        symbol_ty_family: DeclarativeTermFamily,
    ) -> u8 {
        let mut idx = 0;
        let mut t = |term: DeclarativeTerm| {
            if let Some(subidx) = term.new_variable_idx_aux(db, symbol, symbol_ty_family) {
                if subidx > idx {
                    idx = subidx
                }
            }
        };
        match self {
            DeclarativeTerm::Curry(_) => todo!(),
            DeclarativeTerm::Ritchie(term) => {
                for parameter_ty in term.parameter_tys(db) {
                    t(parameter_ty.ty())
                }
                t(term.return_ty(db));
            }
            DeclarativeTerm::Abstraction(term) => {
                let x = term.x(db);
                let m = term.m(db);
                t(m);
                if x.ty_family(db) == symbol_ty_family && m.contains_symbol(db, symbol) {
                    let x_idx = x.idx(db);
                    if x_idx > idx {
                        idx = x_idx
                    }
                }
            }
            DeclarativeTerm::ExplicitApplication(term) => {
                t(term.function(db));
                t(term.argument(db))
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            DeclarativeTerm::Subentity(_) => todo!(),
            DeclarativeTerm::AsTraitSubentity(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            _ => (),
        }
        idx
    }

    // todo: needs thorough testing
    /// not cached on purpose
    fn substitute_symbol_with_variable(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        variable: DeclarativeTermVariable,
    ) -> Self {
        if !self.contains_symbol(db, symbol) {
            return self;
        }
        match self {
            DeclarativeTerm::Symbol(term) if term == symbol => variable.into(),
            DeclarativeTerm::Universe(_) => self, // ad hoc
            DeclarativeTerm::Curry(term) => DeclarativeTermCurry::new(
                db,
                term.curry_kind(db),
                term.variance(db),
                term.parameter_variable(db),
                term.parameter_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
                term.return_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            DeclarativeTerm::Ritchie(term) => DeclarativeTermRitchie::new(
                db,
                term.ritchie_kind(db),
                term.parameter_tys(db)
                    .iter()
                    .map(|parameter_ty| {
                        parameter_ty.substitute_ty(|ty| {
                            ty.substitute_symbol_with_variable(db, symbol, variable)
                        })
                    })
                    .collect(),
                term.return_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            DeclarativeTerm::Abstraction(term) => {
                let x = term.x(db);
                // should be equal by the choice of variable idx and the fact that m contains the symbol
                debug_assert_ne!(x, variable);
                DeclarativeTermAbstraction::new(
                    db,
                    x,
                    term.m(db)
                        .substitute_symbol_with_variable(db, symbol, variable),
                )
                .into()
            }
            DeclarativeTerm::ExplicitApplication(term) => DeclarativeTermExplicitApplication::new(
                db,
                term.function(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
                term.argument(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            DeclarativeTerm::Subentity(_) => todo!(),
            DeclarativeTerm::AsTraitSubentity(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            _ => self,
        }
    }
}

fn variable_registry(
    db: &dyn DeclarativeTermDb,
    raw_term: DeclarativeTerm,
    symbol: DeclarativeTermSymbol,
) -> VariableRegistry {
    todo!()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct VariableRegistry {
    next: u8,
}

impl VariableRegistry {
    fn new(
        db: &dyn DeclarativeTermDb,
        variables: Option<DeclarativeTermVariables>,
        ty_family: DeclarativeTermFamily,
    ) -> Self {
        let Some(variables) = variables else {
            return Default::default()
        };
        let mut next = 0;
        for variable in variables.unaccounted_variables(db).iter().copied() {
            // only need to disambiguous those with the same type family
            if variable.ty_family(db) == ty_family {
                // make sure that new variable won't conflict with this one
                let idx = variable.idx(db);
                if next <= idx {
                    next = idx + 1
                }
            }
        }
        Self { next }
    }

    pub(super) fn issue_variable_idx(self) -> u8 {
        self.next
    }

    fn merge(&mut self, other: Self) {
        todo!()
    }
}