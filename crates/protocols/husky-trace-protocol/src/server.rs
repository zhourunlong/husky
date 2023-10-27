use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
};

use crate::{
    cache::action::{TraceCacheNewTrace, TraceCacheSetSubtraces, TraceCacheToggleExpansion},
    id_map::TraceIdMap,
    message::{TraceRequest, TraceResponse},
    view::{action::TraceViewAction, TraceViewData},
    *,
};
use husky_visual_protocol::{IsVisualComponent, IsVisualProtocol};
use husky_websocket_utils::easy_server::{easy_serve, IsEasyWebsocketServer};

pub struct TraceServer<Tracetime: IsTracetime> {
    cache: Option<TraceCache<Tracetime::VisualComponent>>,
    tracetime: Tracetime,
    trace_id_map: TraceIdMap<Tracetime::Trace>,
}

impl<Tracetime: IsTracetime> Default for TraceServer<Tracetime>
where
    Tracetime: Default,
{
    fn default() -> Self {
        Self {
            cache: Default::default(),
            tracetime: Default::default(),
            trace_id_map: Default::default(),
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn new(tracetime: Tracetime) -> Self {
        Self {
            cache: Default::default(),
            tracetime,
            trace_id_map: Default::default(),
        }
    }

    fn init(&mut self) {
        if self.cache.is_some() {
            return;
        }
        let traces = self.tracetime.get_root_traces();
        self.cache = Some(TraceCache::new(traces.iter().map(|&trace| {
            (
                self.trace_id_map.trace_id_or_alloc_new(trace, |_| ()),
                self.tracetime.get_trace_view_data(trace).clone(),
            )
        })))
    }

    #[track_caller]
    fn cache(&self) -> &TraceCache<Tracetime::VisualComponent> {
        self.cache.as_ref().unwrap()
    }

    #[track_caller]
    fn cache_mut(&mut self) -> &mut TraceCache<Tracetime::VisualComponent> {
        self.cache.as_mut().unwrap()
    }
}

impl<Tracetime: IsTracetime> IsEasyWebsocketServer for TraceServer<Tracetime> {
    type Response = TraceResponse<Tracetime::VisualComponent>;

    type Request = TraceRequest<Tracetime::VisualComponent>;

    type SerdeImpl = Tracetime::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init => {
                self.init();
                let Some(cache) = self.cache.clone() else {
                    unreachable!()
                };
                Some(TraceResponse::Init { cache })
            }
            TraceRequest::TakeViewAction {
                view_action,
                cache_actions_len,
            } => {
                let Some(ref mut cache) = self.cache else {
                    unreachable!()
                };
                assert_eq!(self.cache().actions_len(), cache_actions_len);
                self.take_view_action(view_action);
                Some(TraceResponse::TakeCacheAction {
                    cache_actions: self.cache().reproduce_cache_actions(cache_actions_len),
                })
            }
            TraceRequest::NotifyViewAction {
                view_action,
                cache_action,
            } => {
                self.cache_mut().take_action(cache_action);
                None
            }
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn take_view_action(&mut self, view_action: TraceViewAction<Tracetime::VisualComponent>) {
        match view_action {
            TraceViewAction::ToggleExpansion { trace_id } => {
                assert!(!self.cache()[trace_id].expanded());
                // todo: handle more cases like subtraces with channels
                if self.cache()[trace_id].subtrace_ids().is_some() {
                    return;
                }
                let trace = self.trace_id_map.trace(trace_id);
                let subtraces = self.tracetime.get_subtraces(trace);
                let subtrace_ids = subtraces
                    .iter()
                    .map(|&subtrace| {
                        self.trace_id_map
                            .trace_id_or_alloc_new(subtrace, |subtrace_id| {
                                let view_data = self.tracetime.get_trace_view_data(subtrace);
                                self.cache
                                    .as_mut()
                                    .unwrap()
                                    .take_action(TraceCacheNewTrace::new(subtrace_id, view_data))
                            })
                    })
                    .collect();
                self.cache_mut()
                    .take_action(TraceCacheSetSubtraces::new(trace_id, subtrace_ids));
                self.cache_mut()
                    .take_action(TraceCacheToggleExpansion::new(trace_id))
            }
            TraceViewAction::Marker { _marker } => todo!(),
        }
    }
}

pub trait IsTracetime: Send + 'static + Sized {
    type Trace: Send + Eq + std::hash::Hash + Copy;

    type VisualComponent: IsVisualComponent;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl ToSocketAddrs) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_root_traces(&self) -> &[Self::Trace];

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> TraceViewData;
}