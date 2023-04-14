use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryShiftOpr,
        menu: &TermMenu,
    ) -> ExprTypeResult<FluffyTerm> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        match lopd_ty.data(self) {
            FluffyTermData::TypeOntology {
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::Hole(
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType,
                _,
            ) => {
                self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
                Ok(lopd_ty)
            }
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path: Right(PreludeTypePath::Num(_)),
                arguments: argument_tys,
            } => {
                self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
                Ok(TermEntityPath::TypeOntology(path).into())
            }
            FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Literal(_)
            | FluffyTermData::Curry { .. }
            | FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }

    pub(super) fn calc_num_ty_binary_shift_ropd_ty(&mut self, ropd: ExprIdx) -> ExprTypeResult<()> {
        let Some(ropd_ty) = self.infer_new_expr_ty(ropd, ExpectAnyOriginal) else {
            Err(DerivedExprTypeError::BinaryShiftRightOperandTypeNotInferred)?
        };
        match ropd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                refined_path: Right(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            }
            | FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => Ok(()),
            FluffyTermData::TypeOntology { .. } => todo!(),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}