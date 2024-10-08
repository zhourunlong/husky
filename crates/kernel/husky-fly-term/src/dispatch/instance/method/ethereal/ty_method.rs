use super::*;
use assoc_item::ty_item::method_ritchie::ty_method_fly_signature;
use husky_entity_path::path::major_item::ty::{
    PreludeIndirectionTypePath, PreludeTypePath, TypePath,
};
use husky_eth_term::context::EthTermContextItd;
use husky_eth_term::term::application::{EthApplication, TermFunctionReduced};

impl HasFlyTypeMethodDispatch for EthTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        // todo: check scope
        match self {
            EthTerm::ItemPath(ItemPathTerm::TypeOntology(ty_path)) => {
                ethereal_ty_ontology_path_ty_method_dispatch(
                    engine,
                    expr_idx,
                    ty_path,
                    ident_token,
                    indirections,
                    engine.context_itd(),
                )
            }
            EthTerm::Application(ty_term) => ethereal_term_application_ty_method_dispatch(
                engine,
                expr_idx,
                ty_term,
                ident_token,
                indirections,
                engine.context_itd(),
            ),
            _ => Nothing,
        }
    }
}

fn ethereal_ty_ontology_path_ty_method_dispatch<'db>(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    ident_token: IdentRegionalToken,
    indirections: FlyIndirections,
    context_itd: EthTermContextItd,
) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
    ethereal_ty_method_dispatch_aux(
        engine,
        expr_idx,
        ty_path,
        &[],
        ident_token,
        indirections,
        context_itd,
    )
}

fn ethereal_term_application_ty_method_dispatch<'db>(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    ty_term: EthApplication,
    ident_token: IdentRegionalToken,
    indirections: FlyIndirections,
    context_itd: EthTermContextItd,
) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
    let application_expansion = ty_term.application_expansion(engine.db());
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_method_dispatch_aux(
            engine,
            expr_idx,
            ty_path,
            application_expansion.arguments(engine.db()),
            ident_token,
            indirections,
            context_itd,
        ),
        TermFunctionReduced::TypeVar(_) => todo!(),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_method_dispatch_aux<'db>(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    arguments: &[EthTerm],
    ident_token: IdentRegionalToken,
    mut indirections: FlyIndirections,
    context_itd: EthTermContextItd,
) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
    match ty_path.refine(engine.db()) {
        Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
            match prelude_indirection_ty_path {
                PreludeIndirectionTypePath::Ref => todo!(),
                PreludeIndirectionTypePath::RefMut => todo!(),
                PreludeIndirectionTypePath::Leash => {
                    indirections.add(FlyIndirection::Leash);
                    if arguments.len() != 1 {
                        p!((&arguments).debug(engine.db()));
                        todo!()
                    }
                    return arguments[0].ty_method_dispatch(
                        engine,
                        expr_idx,
                        ident_token,
                        indirections,
                    );
                }
                PreludeIndirectionTypePath::At => todo!(),
            }
        }
        _ => (),
    }
    if let Some(signature) = ty_method_fly_signature(
        engine,
        expr_idx,
        ty_path,
        arguments,
        /* ad hoc */ &[],
        ident_token,
        indirections.final_place(),
        context_itd,
    )
    .into_result_option()?
    {
        return JustOk(FlyInstanceDispatch {
            indirections,
            signature: signature.into(),
        });
    };
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
