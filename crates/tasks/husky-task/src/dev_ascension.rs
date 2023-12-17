use crate::*;
use husky_task_prelude::{value::IsTaskValue, IsLinkageImpl};
use husky_task_prelude::{DevEvalContext, IsDevRuntime, IsDevRuntimeDyn};
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_vfs::VfsDb;
use std::{cell::Cell, thread::LocalKey};

pub trait IsDevAscension {
    type Linktime: IsLinktime;
    type RuntimeStorage: Default + Send;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol;
    fn dev_eval_context_local_key(
    ) -> &'static LocalDevEvalContext<<Self::Linktime as IsLinktime>::LinkageImpl>;
}

pub type LocalDevEvalContext<LinkageImpl> =
    LocalKey<Cell<std::option::Option<DevEvalContext<LinkageImpl>>>>;

pub fn dev_eval_context<DevAscension: IsDevAscension>(
) -> DevEvalContext<<DevAscension::Linktime as IsLinktime>::LinkageImpl> {
    DevAscension::dev_eval_context_local_key()
        .get()
        .expect("`DEV_EVAL_CONTEXT` not set")
}

pub fn with_dev_eval_context<DevAscension: IsDevAscension, R>(
    ctx: DevEvalContext<<DevAscension::Linktime as IsLinktime>::LinkageImpl>,
    f: impl FnOnce() -> R,
) -> R {
    let local_key = DevAscension::dev_eval_context_local_key();
    let old = local_key.replace(Some(ctx));
    let r = f();
    local_key.set(old);
    r
}

pub fn with_runtime_and_base_point<
    DevAscension: IsDevAscension,
    Runtime: IsDevRuntime<<DevAscension::Linktime as IsLinktime>::LinkageImpl>,
    R,
>(
    runtime: &Runtime,
    base_point: <<DevAscension::Linktime as IsLinktime>::LinkageImpl as IsLinkageImpl>::BasePoint,
    f: impl FnOnce() -> R,
) -> R {
    let local_dev_eval_context = DevAscension::dev_eval_context_local_key();
    with_dev_eval_context::<DevAscension, _>(
        DevEvalContext::new(
            unsafe { runtime.cast_to_static_self_static_ref() },
            base_point,
        ),
        f,
    )
}
