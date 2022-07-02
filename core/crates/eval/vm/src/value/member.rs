use serde::Serialize;

use crate::*;

#[derive(Debug, Clone)]
pub enum MemberValue<'eval> {
    Copyable(CopyableValue),
    Boxed(OwnedValue<'eval, 'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(EvalRef<'eval>),
    Moved,
}

impl<'eval> PartialEq for MemberValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copyable(l0), Self::Copyable(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::EvalRef(l0), Self::EvalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for MemberValue<'eval> {}

impl<'temp, 'eval: 'temp> MemberValue<'eval> {
    pub fn into_stack(self) -> __TempValue<'temp, 'eval> {
        match self {
            MemberValue::Copyable(value) => __TempValue::Copyable(value),
            MemberValue::Boxed(value) => __TempValue::OwnedEval(value),
            MemberValue::GlobalPure(value) => __TempValue::EvalPure(value),
            MemberValue::EvalRef(value) => __TempValue::EvalRef(value),
            MemberValue::Moved => panic!(),
        }
    }

    pub fn any_ref<'a>(&'a self) -> &'a (dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            MemberValue::Copyable(value) => value.any_ref(),
            MemberValue::Boxed(value) => value.any_ref(),
            MemberValue::GlobalPure(value) => &**value,
            MemberValue::EvalRef(value) => value.0,
            MemberValue::Moved => panic!(),
        }
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval> + 'eval) {
        self.any_ref()
    }

    pub fn bind(&self, binding: Binding) -> __TempValue<'temp, 'eval> {
        match binding {
            Binding::EvalRef => self.bind_eval_ref(),
            Binding::TempRef => self.bind_temp_ref(),
            Binding::TempRefMut => todo!(),
            Binding::Move => todo!(),
            Binding::Copy => match self {
                MemberValue::Copyable(value) => __TempValue::Copyable(*value),
                MemberValue::Boxed(_) => todo!(),
                MemberValue::GlobalPure(_) => todo!(),
                MemberValue::EvalRef(_) => todo!(),
                MemberValue::Moved => todo!(),
            },
        }
    }

    pub fn bind_eval_ref(&self) -> __TempValue<'temp, 'eval> {
        match self {
            MemberValue::EvalRef(value) => __TempValue::EvalRef(*value),
            MemberValue::Copyable(_) => panic!("can't bind eval ref to a copyable value"),
            MemberValue::Boxed(ref boxed_value) => {
                __TempValue::EvalRef(EvalRef(unsafe { &*boxed_value.any_ptr() }))
            }
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn bind_temp_ref(&self) -> __TempValue<'temp, 'eval> {
        match self {
            MemberValue::Boxed(boxed_value) => {
                __TempValue::TempRefEval(unsafe { &*boxed_value.any_ptr() })
            }
            MemberValue::Copyable(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(value) => __TempValue::TempRefEval(value.0),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn bind_mut<'a>(&'a mut self, owner: VMStackIdx) -> __TempValue<'temp, 'eval> {
        let value_mut: *mut dyn AnyValueDyn<'eval> = match self {
            MemberValue::Copyable(value) => value.any_mut(),
            MemberValue::Boxed(value) => value.any_mut_ptr(),
            _ => todo!(),
        };
        __TempValue::TempRefMutEval {
            value: unsafe { &mut *value_mut },
            owner,
            gen: (),
        }
    }

    pub fn share_globally(&'eval self) -> EvalValue<'eval> {
        match self {
            MemberValue::Copyable(value) => EvalValue::Copyable(*value),
            MemberValue::Boxed(value) => EvalValue::EvalRef(EvalRef(value.any_ref())),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn copy_into_stack(&self) -> __TempValue<'temp, 'eval> {
        match self {
            MemberValue::Copyable(value) => __TempValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }
}

// impl<'eval, 'a: 'eval> __AnyValue<'eval> for MemberValue<'a> {
//     fn static_type_id() -> StaticTypeId {
//         StaticTypeId::AnyMemberValue
//     }

//     fn static_type_name() -> std::borrow::Cow<'static, str> {
//         "AnyMemberValue".into()
//     }

//     fn to_json_value(&self) -> serde_json::value::Value {
//         self.any_ref().to_json_value_dyn()
//     }

//     fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
//     where
//         'eval: 'short,
//     {
//         self
//     }

//     fn ty(&self) -> husky_entity_route::EntityRoutePtr {
//         self.any_ref().ty_dyn()
//     }
// }
