pub mod error;
#[cfg(feature = "mock")]
pub mod mock;

use crate::{
    cache::action::TraceCacheToggleExpansion, message::*, view::action::TraceViewAction, *,
};
use husky_websocket_utils::imgui_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use notify::Notify;
use std::sync::Arc;

pub struct TraceClient<TraceProtocol: IsTraceProtocol, Notifier>
where
    Notifier: Notify,
{
    opt_cache: Option<TraceCache<TraceProtocol>>,
    connection: ImmediateWebsocketClientConnection<
        TraceRequest<TraceProtocol>,
        TraceResponse<TraceProtocol>,
        Notifier,
    >,
}

impl<TraceProtocol: IsTraceProtocol, Notifier> TraceClient<TraceProtocol, Notifier>
where
    TraceProtocol: IsTraceProtocol,
    Notifier: Notify,
{
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        server_address: impl Into<String>,
        notifier: Notifier,
    ) -> Self {
        Self {
            opt_cache: None,
            connection: ImmediateWebsocketClientConnection::new(
                tokio_runtime,
                server_address.into(),
                notifier,
            ),
        }
    }

    pub fn update(&mut self) {
        let Some(response) = self.connection.try_recv() else {
            return;
        };
        self.process_response(response);
    }

    fn process_response(&mut self, response: TraceResponse<TraceProtocol>) {
        match response {
            TraceResponse::Init { cache } => {
                debug_assert!(self.opt_cache.is_none());
                self.opt_cache = Some(cache)
            }
            TraceResponse::TakeCacheAction { cache_actions } => {
                let Some(ref mut cache) = self.opt_cache else {
                    unreachable!()
                };
                cache.take_actions(cache_actions)
            }
        }
    }

    fn try_send_request(
        &mut self,
        request: TraceRequest<TraceProtocol>,
    ) -> Result<(), WebsocketClientConnectionError> {
        self.connection.try_send_request(request)
    }

    pub fn root_trace_ids(&self) -> Option<&[TraceId]> {
        Some(self.opt_cache.as_ref()?.root_trace_ids())
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }

    pub fn opt_cache(&self) -> Option<&TraceCache<TraceProtocol>> {
        self.opt_cache.as_ref()
    }

    #[track_caller]
    fn cache(&self) -> &TraceCache<TraceProtocol> {
        self.opt_cache.as_ref().unwrap()
    }

    #[track_caller]
    fn cache_mut(&mut self) -> &mut TraceCache<TraceProtocol> {
        self.opt_cache.as_mut().unwrap()
    }

    pub fn take_view_action(
        &mut self,
        view_action: TraceViewAction<TraceProtocol>,
    ) -> Result<(), WebsocketClientConnectionError> {
        let Some(cache_action) = self.try_resolve_view_action(&view_action) else {
            let cache_actions_len = self.cache().actions_len();
            return self.try_send_request(TraceRequest::TakeViewAction {
                view_action,
                cache_actions_len,
            });
        };
        self.cache_mut().take_action(cache_action.clone());
        match self.try_send_request(TraceRequest::NotifyViewAction {
            view_action,
            cache_action,
        }) {
            Ok(_) => (),
            Err(e) => match e {
                WebsocketClientConnectionError::SendRequestWhileCreation => todo!(),
                WebsocketClientConnectionError::SendRequestWhileDeserializingRequest => todo!(),
                WebsocketClientConnectionError::SendRequestWhileAwaitingResponse => todo!(),
                WebsocketClientConnectionError::SendRequestWhileSerializingResponse => todo!(),
                WebsocketClientConnectionError::SendRequestWhileResponseNotProcessed => {
                    let response = self.connection.try_recv();
                    todo!("SendRequestWhileResponseNotProcessed, response = {response:?}")
                }
            },
        }
        return Ok(());
    }

    fn try_resolve_view_action(
        &self,
        view_action: &TraceViewAction<TraceProtocol>,
    ) -> Option<TraceCacheAction<TraceProtocol>> {
        match view_action {
            &TraceViewAction::ToggleExpansion { trace_id } => {
                let trace_cache_entry = &self.cache()[trace_id];
                if !trace_cache_entry.expanded() {
                    trace_cache_entry.subtrace_ids()?;
                }
                Some(TraceCacheToggleExpansion::new(trace_id).into())
            }
            TraceViewAction::Marker { _marker } => todo!(),
            TraceViewAction::ToggleExpansion { trace_id: _ } => todo!(),
            &TraceViewAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                if !self.cache().is_trace_cached(associated_trace_id) {
                    return None;
                }
                Some(TraceCacheAction::ToggleAssociatedTrace {
                    trace_id,
                    associated_trace_id,
                })
            }
        }
    }
}
