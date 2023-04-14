use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectAnyOriginal;

impl ExpectFluffyTerm for ExpectAnyOriginal {
    type Outcome = FluffyTerm;

    fn retrieve_outcome(_outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        todo!()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        FinalDestination::AnyOriginal
    }

    fn destination(&self) -> Option<FluffyTerm> {
        None
    }
}
