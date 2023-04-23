use super::*;

pub(super) fn direct_ty_memo_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeDeclarativeSignatureTemplate,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyCardResult<Option<FluffyFieldCard>> {
    // ad hoc
    Ok(None)
}