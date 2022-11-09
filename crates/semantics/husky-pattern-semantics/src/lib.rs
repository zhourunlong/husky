use husky_entity_route::EntityRouteItd;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_primitive_literal_syntax::RawLiteralData;
use husky_term_infer::TermInferDb;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurePattern {
    pub ty: EntityRouteItd,
    pub variant: PurePatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PurePatternVariant {
    PrimitiveLiteral(RawLiteralData),
    OneOf { subpatterns: Vec<PurePattern> },
    EnumLiteral(EntityRouteItd),
    Some,
    None,
}

impl PurePattern {
    pub fn from_raw(db: &dyn TermInferDb, raw_patt: &RawPattern, ty: EntityRouteItd) -> Self {
        let variant = match raw_patt.variant {
            RawPatternVariant::PrimitiveLiteral(data) => PurePatternVariant::PrimitiveLiteral(data),
            RawPatternVariant::OneOf { ref subpatterns } => PurePatternVariant::OneOf {
                subpatterns: subpatterns
                    .iter()
                    .map(|subpattern| PurePattern::from_raw(db, subpattern, ty))
                    .collect(),
            },
            RawPatternVariant::EnumLiteral(_) => todo!(),
            RawPatternVariant::Some => PurePatternVariant::Some,
            RawPatternVariant::None => PurePatternVariant::None,
        };
        Self { ty, variant }
    }
}
