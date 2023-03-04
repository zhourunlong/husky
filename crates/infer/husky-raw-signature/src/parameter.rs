use husky_expr::{ExplicitParameterDeclPattern, ImplicitParameterDeclPatternVariant};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterRawSignature {
    annotated_variance: Option<Variance>,
    symbol: RawTermSymbol,
    ty: RawTerm,
    traits: Vec<RawTerm>,
}

impl ImplicitParameterRawSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &RawSignatureRawTermRegion,
        raw_term_menu: &RawTermMenu,
    ) -> ImplicitParameterRawSignature {
        let pattern = &parameter_decl.pattern();
        let symbol = pattern.symbol();
        let annotated_variance = pattern.annotated_variance_token().map(|t| match t {
            VarianceToken::Covariant(_) => Variance::Covariant,
            VarianceToken::Contravariant(_) => Variance::Contravariant,
            VarianceToken::Invariant(_) => Variance::Invariant,
        });
        match parameter_decl.pattern().variant() {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterRawSignature {
                    symbol: region.current_symbol_term(symbol).expect("not none"),
                    ty: raw_term_menu.ty0().into(),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterRawSignature {
                    symbol: region.current_symbol_term(symbol).expect("not none"),
                    ty: raw_term_menu.lifetime_ty().into(),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Binding { .. } => todo!(),
        }
    }

    pub fn symbol(&self) -> RawTermSymbol {
        self.symbol
    }

    pub fn ty(&self) -> RawTerm {
        self.ty
    }

    pub fn traits(&self) -> &[RawTerm] {
        self.traits.as_ref()
    }

    pub fn annotated_variance(&self) -> Option<Variance> {
        self.annotated_variance
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterRawSignatures {
    data: Vec<ImplicitParameterRawSignature>,
}

impl ImplicitParameterRawSignatures {
    pub(crate) fn from_decl(
        implicit_parameters: &[ImplicitParameterDecl],
        raw_signature_term_region: &RawSignatureRawTermRegion,
        raw_term_menu: &RawTermMenu,
    ) -> Self {
        Self {
            data: implicit_parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterRawSignature::from_decl(
                        parameter,
                        raw_signature_term_region,
                        raw_term_menu,
                    )
                })
                .collect(),
        }
    }

    pub fn decls(&self) -> &[ImplicitParameterRawSignature] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for ImplicitParameterRawSignatures {
    type Target = Vec<ImplicitParameterRawSignature>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterRawSignature {
    pattern: ParameterRawSignaturePattern,
    ty: RawTerm,
}

impl ParameterRawSignature {
    pub fn ty(&self) -> RawTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterRawSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RegularParameterRawSignatures {
    parameters: Vec<ParameterRawSignature>,
}

impl std::ops::Deref for RegularParameterRawSignatures {
    type Target = Vec<ParameterRawSignature>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl RegularParameterRawSignatures {
    pub(crate) fn from_decl(
        parameters: &[ExplicitParameterDeclPattern],
        sheet: &RawSignatureRawTermRegion,
    ) -> RawSignatureResult<Self> {
        Ok(Self {
            parameters: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let ty = parameter.ty();
                    let ty = match sheet.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(RawSignatureError::ParameterTypeRawTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    };
                    Ok(ParameterRawSignature {
                        pattern: ParameterRawSignaturePattern {},
                        ty,
                    })
                })
                .collect::<Result<Vec<ParameterRawSignature>, RawSignatureError>>()?,
        })
    }
}