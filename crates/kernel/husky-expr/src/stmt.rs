use husky_entity_path::path::major_item::ty::PreludeIntTypePath;
use husky_opr::BinaryComparisonOpr;

/// loop step
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LoopStep {
    Constant(i32),
}

pub struct LoopStepValue(pub i32);

impl LoopStepValue {
    pub fn n(&self, a: i32, b: i32) -> i32 {
        if (b - a) * self.0 >= 0 {
            (b - a) / self.0 + 1
        } else {
            0
        }
    }

    pub fn for_loop_variable(&self, a: i32, i: i32) -> i32 {
        a + self.0 * i
    }
}

#[test]
fn test_step_n_for_pos_step() {
    let step = LoopStepValue(1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, 1), 2);
    assert_eq!(step.n(0, 2), 3);
    assert_eq!(step.n(0, -1), 0);
}

#[test]
fn test_step_n_for_neg_step() {
    let step = LoopStepValue(-1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, -1), 2);
    assert_eq!(step.n(0, -2), 3);
    assert_eq!(step.n(0, -3), 4);
    assert_eq!(step.n(0, 1), 0);
}

/// loop boundary kind
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LoopBoundaryKind {
    UpperOpen,
    UpperClosed,
    LowerOpen,
    LowerClosed,
}

impl LoopBoundaryKind {
    pub fn new_final(final_comparison: BinaryComparisonOpr) -> LoopBoundaryKind {
        match final_comparison {
            // ... $for_loop_variable >= $final_bound
            BinaryComparisonOpr::Geq => LoopBoundaryKind::LowerClosed,
            // ... $for_loop_variable > $final_bound
            BinaryComparisonOpr::Greater => LoopBoundaryKind::LowerOpen,
            // ... $for_loop_variable <= $final_bound
            BinaryComparisonOpr::Leq => LoopBoundaryKind::UpperClosed,
            // ... $for_loop_variable < $final_bound
            BinaryComparisonOpr::Less => LoopBoundaryKind::UpperOpen,
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConditionConversion {
    None,
    IntToBool(PreludeIntTypePath),
}
