use super::*;
use idx_arena::{Arena, ArenaIdx};

#[derive(Default, Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct Expectations {
    arena: Arena<FluffyTermExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<FluffyTermExpectationIdx> for Expectations {
    type Output = FluffyTermExpectationEntry;

    fn index(&self, index: FluffyTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl Expectations {
    pub(crate) fn unresolved_expectation_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut FluffyTermExpectationEntry> {
        self.arena
            .iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|entry| match entry.meta.resolve_progress() {
                ExpectationProgress::Intact | ExpectationProgress::Holed => true,
                ExpectationProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &FluffyTermExpectationEntry> {
        self.arena.iter()
    }

    pub(super) fn alloc_expectation(
        &mut self,
        entry: FluffyTermExpectationEntry,
    ) -> FluffyTermExpectationIdx {
        self.arena.alloc_one(entry)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExpectationSource {
    syn_expr_idx: SynExprIdx,
    kind: ExpectationSourceKind,
}

impl ExpectationSource {
    pub fn new_expr(syn_expr_idx: SynExprIdx) -> Self {
        Self {
            syn_expr_idx,
            kind: ExpectationSourceKind::Expr,
        }
    }

    pub(crate) fn child_src(self, idx: FluffyTermExpectationIdx) -> Self {
        Self {
            syn_expr_idx: self.syn_expr_idx,
            kind: ExpectationSourceKind::Expectation(idx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSourceKind {
    Expr,
    Expectation(FluffyTermExpectationIdx),
}

impl ExpectationSource {
    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct FluffyTermExpectationEntry {
    expectation: Expectation,
    meta: ExpectationState,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ExpectationState {
    idx: FluffyTermExpectationIdx,
    src: ExpectationSource,
    expectee: FluffyTerm,
    resolve_progress: ExpectationProgress,
}

impl FluffyTermExpectationEntry {
    pub(crate) fn resolve(
        &mut self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
    ) -> AltOption<FluffyTermEffect> {
        self.expectation.resolve(db, terms, &mut self.meta)
    }

    #[inline]
    pub fn resolve_progress(&self) -> &ExpectationProgress {
        &self.meta.resolve_progress
    }

    #[inline]
    pub fn src(&self) -> ExpectationSource {
        self.meta.src
    }

    #[inline]
    pub fn original_error(&self) -> Option<&OriginalFluffyTermExpectationError> {
        match self.meta.resolve_progress {
            ExpectationProgress::Resolved(Err(FluffyTermExpectationError::Original(ref e))) => {
                Some(e)
            }
            _ => None,
        }
    }
}

impl ExpectationState {
    pub fn src(&self) -> ExpectationSource {
        self.src
    }

    pub(crate) fn child_src(&self) -> ExpectationSource {
        self.src.child_src(self.idx())
    }

    pub(crate) fn expectee(&self) -> FluffyTerm {
        self.expectee
    }

    pub(crate) fn resolve_progress(&self) -> &ExpectationProgress {
        &self.resolve_progress
    }

    pub(crate) fn idx(&self) -> FluffyTermExpectationIdx {
        self.idx
    }

    pub(crate) fn set_holed(
        &mut self,
        hole: Hole,
        gen_hole_constraint: impl FnOnce(&mut Self) -> HoleConstraint,
    ) -> AltOption<FluffyTermEffect> {
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            ExpectationProgress::Holed => return AltNone,
            ExpectationProgress::Intact => (),
        }
        self.resolve_progress = ExpectationProgress::Holed;
        AltSome(FluffyTermEffect {
            subsequent_actions: smallvec![FluffyTermResolveAction::AddHoleConstraint {
                hole,
                hole_constraint: gen_hole_constraint(self)
            }],
        })
    }

    /// returns option for convenience
    pub(crate) fn set_ok(
        &mut self,
        outcome: impl Into<FluffyTermExpectationOutcome>,
        subsequent_actions: FluffyTermResolveActions,
    ) -> AltOption<FluffyTermEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Ok(outcome.into()));
        AltSome(FluffyTermEffect { subsequent_actions })
    }

    pub(crate) fn set_err(
        &mut self,
        e: impl Into<FluffyTermExpectationError>,
        subsequent_actions: FluffyTermResolveActions,
    ) -> AltOption<FluffyTermEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Err(e.into()));
        AltSome(FluffyTermEffect { subsequent_actions })
    }
}

#[derive(Debug, Default)]
pub struct FluffyTermEffect {
    pub(crate) subsequent_actions: FluffyTermResolveActions,
}

impl FluffyTermEffect {
    pub(crate) fn take_subsequent_actions(self) -> FluffyTermResolveActions {
        self.subsequent_actions
    }
}

impl FluffyTermRegion {
    pub fn add_expectation(
        &mut self,
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: impl Into<Expectation>,
    ) -> Option<FluffyTermExpectationIdx> {
        let idx = unsafe { self.expectations.arena.next_idx() };
        Some(
            self.expectations
                .alloc_expectation(FluffyTermExpectationEntry {
                    expectation: expectation.into(),
                    meta: ExpectationState {
                        idx,
                        src,
                        expectee: expectee.into(),
                        resolve_progress: ExpectationProgress::Intact,
                    },
                }),
        )
    }
}
