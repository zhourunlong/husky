use crate::*;

#[salsa::jar]
pub struct ValReprJar(
    KiRepr,
    val_repr_val,
    val_item_val_repr,
    ValReprExpansion,
    val_repr_expansion,
);