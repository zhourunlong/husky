use core::num::NonZeroUsize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SynPrefixOpr {
    Minus,       // -
    Not,         // !$0
    Tilde,       // ~
    Ref,         // &
    Vector,      // []
    Slice,       // [:]
    CyclicSlice, // [%]
}

impl SynPrefixOpr {
    //     pub fn act_on_primitive(&self, opd: PrimitiveValueData) -> PrimitiveValueData {
    //         match self {
    //             PrefixOpr::Minus => match opd {
    //                 PrimitiveValueData::I32(i) => (-i).into(),
    //                 PrimitiveValueData::I64(i) => (-i).into(),
    //                 PrimitiveValueData::F32(f) => (-f).into(),
    //                 PrimitiveValueData::B32(_) => todo!(),
    //                 PrimitiveValueData::B64(_) => todo!(),
    //                 PrimitiveValueData::Bool(_) => todo!(),
    //                 PrimitiveValueData::Unit => panic!(),
    //             },
    //             PrefixOpr::Not => match opd {
    //                 PrimitiveValueData::I32(i) => i == 0,
    //                 PrimitiveValueData::I64(i) => i == 0,
    //                 PrimitiveValueData::F32(f) => f == 0.,
    //                 PrimitiveValueData::B32(b) => b == 0,
    //                 PrimitiveValueData::B64(b) => b == 0,
    //                 PrimitiveValueData::Bool(b) => !b,
    //                 PrimitiveValueData::Unit => panic!(),
    //             }
    //             .into(),
    //             PrefixOpr::BitNot => match opd {
    //                 PrimitiveValueData::B32(b) => (!b).into(),
    //                 PrimitiveValueData::B64(b) => (!b).into(),
    //                 _ => panic!(),
    //             },
    //             PrefixOpr::Shared => todo!(),
    //             PrefixOpr::Move => todo!(),
    //         }
    //     }

    pub fn code(self) -> std::borrow::Cow<'static, str> {
        match self {
            SynPrefixOpr::Minus => "-".into(),
            SynPrefixOpr::Not => "!".into(),
            SynPrefixOpr::Tilde => "!".into(),
            SynPrefixOpr::Ref => "&".into(),
            SynPrefixOpr::Vector => "[]".into(),
            SynPrefixOpr::Slice => "[:]".into(),
            SynPrefixOpr::CyclicSlice => "[%]".into(),
        }
    }
}