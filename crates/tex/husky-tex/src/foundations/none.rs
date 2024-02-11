use std::fmt::{self, Debug, Formatter};

use ecow::EcoString;
use serde::{Serialize, Serializer};

use crate::diag::StrResult;
use crate::foundations::{
    cast, ty, CastInfo, FromTexValue, IntoTexValue, Reflect, Repr, TexValue, Type,
};

/// A value that indicates the absence of any other value.
///
/// The none type has exactly one value: `{none}`.
///
/// When inserted into the document, it is not visible. This is also the value
/// that is produced by empty code blocks. It can be
/// [joined]($scripting/#blocks) with any value, yielding the other value.
///
/// # Example
/// ```example
/// Not visible: #none
/// ```
#[ty(cast, name = "none")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NoneTexValue;

impl Reflect for NoneTexValue {
    fn input() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn output() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn castable(value: &TexValue) -> bool {
        matches!(value, TexValue::None)
    }
}

impl IntoTexValue for NoneTexValue {
    fn into_value(self) -> TexValue {
        TexValue::None
    }
}

impl FromTexValue for NoneTexValue {
    fn from_value(value: TexValue) -> StrResult<Self> {
        match value {
            TexValue::None => Ok(Self),
            _ => Err(Self::error(&value)),
        }
    }
}

impl Debug for NoneTexValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("None")
    }
}

impl Repr for NoneTexValue {
    fn repr(&self) -> EcoString {
        "none".into()
    }
}

impl Serialize for NoneTexValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

cast! {
    (),
    self => TexValue::None,
    _: NoneTexValue => (),
}

impl<T: Reflect> Reflect for Option<T> {
    fn input() -> CastInfo {
        T::input() + NoneTexValue::input()
    }

    fn output() -> CastInfo {
        T::output() + NoneTexValue::output()
    }

    fn castable(value: &TexValue) -> bool {
        NoneTexValue::castable(value) || T::castable(value)
    }
}

impl<T: IntoTexValue> IntoTexValue for Option<T> {
    fn into_value(self) -> TexValue {
        match self {
            Some(v) => v.into_value(),
            None => TexValue::None,
        }
    }
}

impl<T: FromTexValue> FromTexValue for Option<T> {
    fn from_value(value: TexValue) -> StrResult<Self> {
        match value {
            TexValue::None => Ok(None),
            v if T::castable(&v) => Ok(Some(T::from_value(v)?)),
            _ => Err(Self::error(&value)),
        }
    }
}