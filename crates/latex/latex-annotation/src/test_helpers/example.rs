use crate::{
    annotation::{
        space::{LxApplyAnnotation, LxSpaceAnnotation},
        token::{LxTokenAnnotation, LxVariableAnnotation},
    },
    annotations::LxAnnotations,
};

pub struct LxAnnotationsExample {
    pub root_mode: LxMode,
    pub input: String,
    pub annotations: LxAnnotations,
}

pub type LxAnnotationsExamples = Vec<LxAnnotationsExample>;

impl LxAnnotationsExample {
    fn collect_from_sparse(
        examples: &[(
            LxMode,
            &str,
            &[((&str, &str), LxTokenAnnotation)],
            &[((&str, &str), LxSpaceAnnotation)],
        )],
    ) -> LxAnnotationsExamples {
        examples
            .iter()
            .map(
                |&(root_mode, input, token_annotations, space_annotations)| Self {
                    root_mode,
                    input: input.to_string(),
                    annotations: LxAnnotations::from_sparse(
                        input,
                        token_annotations.iter().copied(),
                        space_annotations.iter().copied(),
                    ),
                },
            )
            .collect()
    }
}

use latex_prelude::mode::LxMode;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EXAMPLES: LxAnnotationsExamples = LxAnnotationsExample::collect_from_sparse(&[
        (LxMode::Math, "", &[], &[]),
        (
            LxMode::Math,
            "xy",
            &[
                (
                    ("", "x"),
                    LxTokenAnnotation::Variable(LxVariableAnnotation::Usage)
                ),
                (
                    ("x", "y"),
                    LxTokenAnnotation::Variable(LxVariableAnnotation::Usage)
                ),
            ],
            &[(
                ("x", "y"),
                LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarMul)
            ),]
        ),
        (
            LxMode::Math,
            "dx",
            &[
                (("", "d"), LxTokenAnnotation::Differential),
                (
                    ("d", "x"),
                    LxTokenAnnotation::Variable(
                        LxVariableAnnotation::SingleVariableIntegralVariableDecl
                    )
                ),
            ],
            &[]
        ),
    ]);
}

#[test]
fn latex_annotations_examples_works() {
    let examples = &*EXAMPLES; // Dereference the lazy_static
}
