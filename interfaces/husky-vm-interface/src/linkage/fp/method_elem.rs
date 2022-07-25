#[macro_export]
macro_rules! method_elem_copy_fp {
    ($Type: ty, $method_name: ident) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref();
            todo!()
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! method_elem_eval_ref_fp {
    ($Type: ty, $method_name: ident) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            __Register::new_eval_ref(this_value.$method_name())
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! method_elem_temp_ref_fp {
    ($Type: ty, $method_name: ident) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref();
            __Register::new_temp_ref(this_value.$method_name())
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! method_elem_move_fp {
    ($Type: ty, $method_name: ident) => {{
        __LinkageFp {
            wrapper: |_, values| -> __Register { todo!("move") },
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! method_elem_temp_mut_fp {
    ($Type: ty, $method_name: ident) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &mut $Type = values[0].downcast_temp_mut();
            todo!()
            // Ok(__TempValue::TempRefMutEval {
            //     value: &mut this_value[method_elem_value],
            //     owner,
            //     gen: (),
            // })
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}
