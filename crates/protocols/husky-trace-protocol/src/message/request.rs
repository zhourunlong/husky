#[cfg(feature = "client")]
use husky_websocket_utils::imgui_client::NeedResponse;

use crate::view::action::TraceViewAction;

use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceRequest<VisualComponent> {
    Init,
    /// view action are not handled on client side,
    /// ask the server to handle it and return cache actions
    TakeViewAction {
        view_action: TraceViewAction<VisualComponent>,
        cache_actions_len: usize,
    },
    /// view action already handled on client side,
    /// ask the server to do the same
    NotifyViewAction {
        view_action: TraceViewAction<VisualComponent>,
        cache_action: TraceCacheAction<VisualComponent>,
    },
}

impl<VisualComponent> Default for TraceRequest<VisualComponent> {
    fn default() -> Self {
        TraceRequest::Init
    }
}

#[cfg(feature = "client")]
impl<VisualComponent> NeedResponse for TraceRequest<VisualComponent> {
    fn need_response(&self) -> bool {
        match self {
            TraceRequest::Init => true,
            TraceRequest::TakeViewAction { .. } => true,
            TraceRequest::NotifyViewAction { .. } => false,
        }
    }
}