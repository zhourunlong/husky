use crate::*;
use husky_linket_impl::var::IsVarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StandardVarId {
    data: [u64; 4],
}

impl IsVarId for StandardVarId {}

impl StandardVarId {
    pub fn new(data: [u64; 4]) -> Self {
        Self { data }
    }
}

impl From<u32> for StandardVarId {
    fn from(data: u32) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<u32> for StandardVarId {
    fn into(self) -> u32 {
        self.data[0] as u32
    }
}

impl From<u64> for StandardVarId {
    fn from(data: u64) -> Self {
        Self {
            data: [data, 0, 0, 0],
        }
    }
}

impl Into<u64> for StandardVarId {
    fn into(self) -> u64 {
        self.data[0]
    }
}

impl From<usize> for StandardVarId {
    fn from(data: usize) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<usize> for StandardVarId {
    fn into(self) -> usize {
        self.data[0] as usize
    }
}

impl From<[u64; 4]> for StandardVarId {
    fn from(data: [u64; 4]) -> Self {
        Self { data }
    }
}

#[macro_export]
macro_rules! static_var_linket_impl {
    ($static_var: path, $item_path_id_interface: path) => {
        __LinketImpl::StaticVar {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            get_id: <$static_var as __IsStaticVar<__VarId>>::get_id,
            try_replace_id: |id, locked| unsafe {
                <$static_var as __IsStaticVar<__VarId>>::try_replace_id(id, locked)
                    .map(|restore| -> Box<dyn FnOnce()> { Box::new(restore) })
            },
            ids: |locked| Box::new(<$static_var as __IsStaticVar<__VarId>>::ids(locked)),
            get_value: <$static_var as __IsStaticVar<__VarId>>::get_value,
        }
    };
}

#[test]
fn static_var_linket_impl_works() {
    use crate::{pedestal::StandardPedestal, static_var::StandardVarId, ugly::*};
    use husky_linket_impl::var::IsStaticVar;
    use StandardLinketImpl as __LinketImpl;

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A {}

    impl __IsStaticVar<__VarId> for STATIC_VAR_A {
        fn item_path_id_interface() -> ItemPathIdInterface {
            todo!()
        }

        unsafe fn ids_aux(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = __VarId> {
            (0..10u32).map(Into::into)
        }

        fn get_id() -> __VarId {
            todo!()
        }

        unsafe fn try_replace_id_aux(
            id: __VarId,
            locked: &[ItemPathIdInterface],
        ) -> __StaticVarResult<impl FnOnce() + 'static> {
            todo!();
            Ok(|| todo!())
        }

        type Value = Value;

        fn get_value() -> Self::Value {
            todo!()
        }
    }

    /// We use the same name
    thread_local! {
        pub static STATIC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    #[allow(non_upper_case_globals)]
    pub static mut STATIC_VAR_A__ITEM_PATH_ID_INTERFACE: Option<ItemPathIdInterface> = None;

    let LinketImpl::StaticVar {
        init_item_path_id_interface,
        get_id,
        try_replace_id,
        ids,
        ..
    } = static_var_linket_impl!(STATIC_VAR_A, STATIC_VAR_A__ITEM_PATH_ID_INTERFACE)
    else {
        unreachable!()
    };
}
