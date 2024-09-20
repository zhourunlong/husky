#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};

#[rustfmt::skip]
linket_impls![
    enum_index_presenter_linket_impl!(mnist::MnistLabel),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    static_var_linket_impl!(mnist::INPUT, mnist::__INPUT__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linket_impl!(<mnist::BinaryGrid28>::new_zeros),
    fn_linket_impl!(<mnist::task::MnistTask>::new),
    static_var_linket_impl!(mnist::TASK, mnist::__TASK__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<i8>::abs),
    fn_linket_impl!(<i8>::max),
    fn_linket_impl!(<i8 as Add<i8>>::add),
    fn_linket_impl!(<i16>::abs),
    fn_linket_impl!(<i16>::max),
    fn_linket_impl!(<i16 as Add<i16>>::add),
    fn_linket_impl!(<i32>::abs),
    fn_linket_impl!(<i32>::max),
    fn_linket_impl!(<i32>::min),
    fn_linket_impl!(<i32 as Add<i32>>::add),
    fn_linket_impl!(<i64>::abs),
    fn_linket_impl!(<i64 as Add<i64>>::add),
    fn_linket_impl!(<i128>::abs),
    fn_linket_impl!(<i128 as Add<i128>>::add),
    fn_linket_impl!(<isize>::abs),
    fn_linket_impl!(<isize as Add<isize>>::add),
    fn_linket_impl!(<u8 as Add<u8>>::add),
    fn_linket_impl!(<u16 as Add<u16>>::add),
    fn_linket_impl!(<u32 as Add<u32>>::add),
    fn_linket_impl!(<u64 as Add<u64>>::add),
    fn_linket_impl!(<u128 as Add<u128>>::add),
    fn_linket_impl!(<usize as Add<usize>>::add),
    fn_linket_impl!(<f32>::abs),
    fn_linket_impl!(<f32>::sqrt),
    fn_linket_impl!(<f32>::max),
    fn_linket_impl!(<f32>::min),
    fn_linket_impl!(<f32>::sgnx),
    fn_linket_impl!(<f32>::cos),
    fn_linket_impl!(<f32>::sin),
    fn_linket_impl!(<f32>::acos),
    fn_linket_impl!(<f32 as Add<f32>>::add),
    fn_linket_impl!(<f64>::abs),
    fn_linket_impl!(<f64>::acos),
    fn_linket_impl!(<f64 as Add<f64>>::add),
    fn_linket_impl!(<u32>::last_bits),
    fn_linket_impl!(<u32>::ctz),
    fn_linket_impl!(<u32>::co),
    fn_linket_impl!(<u32>::span),
    fn_linket_impl!(<u32>::right_mass),
    static_var_linket_impl!(husky_core::task::TASK, husky_core::task::__TASK__ITEM_PATH_ID_INTERFACE),
];