use super::*;
use semantics_eager::{EagerExpr, EagerExprVariant, EagerOpnKind};
use syntax_types::SuffixOpr;
use vm::PrimitiveValue;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable(varname) => self.write(&varname),
            EagerExprVariant::This => self.write("self"),
            EagerExprVariant::Scope { scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => self.gen_primitive_literal(value),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_kind,
                ref opds,
            } => match opn_kind {
                EagerOpnKind::Binary { opr, this } => {
                    self.gen_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.gen_expr(&opds[1]);
                }
                EagerOpnKind::Prefix { opr, this } => todo!(),
                EagerOpnKind::Suffix { opr, this } => match opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(field_ident) => {
                        self.gen_expr(&opds[0]);
                        self.write(".");
                        self.write(&field_ident.ident)
                    }
                    SuffixOpr::WithType(_) => todo!(),
                },
                EagerOpnKind::RoutineCall(_) => todo!(),
                EagerOpnKind::TypeCall { ranged_ty, ty_decl } => {
                    self.gen_scope(ranged_ty.route);
                    self.write("::");
                    self.write("__call__(");
                    self.gen_arguments(opds);
                    self.write(")");
                }
                EagerOpnKind::PatternCall => todo!(),
                EagerOpnKind::FieldAccess {
                    field_contract: field_var_contract,
                } => todo!(),
                EagerOpnKind::MethodCall {
                    method_ident: field_ident,
                    ..
                } => {
                    self.gen_expr(&opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident);
                    self.write("(");
                    self.gen_arguments(&opds[1..]);
                    self.write(")");
                }
                EagerOpnKind::ElementAccess => todo!(),
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn gen_arguments(&mut self, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_expr(expr)
        }
    }

    fn gen_primitive_literal(&mut self, v: PrimitiveValue) {
        match v {
            PrimitiveValue::I32(i) => {
                self.result.push_str(&i.to_string());
                self.write("i32")
            }
            PrimitiveValue::F32(_) => todo!(),
            PrimitiveValue::B32(_) => todo!(),
            PrimitiveValue::B64(_) => todo!(),
            PrimitiveValue::Bool(_) => todo!(),
            PrimitiveValue::Void => todo!(),
        }
    }
}
