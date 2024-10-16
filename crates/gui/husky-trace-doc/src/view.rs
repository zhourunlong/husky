mod caryatid;
mod devtools;
mod figure;
mod forest;
mod layout;
pub(crate) mod settings;

use crate::*;
use egui::{
    vec2, Color32, FontFamily, Frame, Label, Margin, RichText, Sense, SidePanel, TextStyle,
    TopBottomPanel, Ui, Vec2, Widget,
};
use husky_trace_protocol::{
    figure::FigureUi,
    protocol::IsTraceProtocol,
    stalk::TraceStalk,
    synchrotron::{TraceSynchrotron, TraceSynchrotronEntry},
    trace_id::{TraceId, TraceKind},
    view::{action::TraceViewActionBuffer, TraceViewLineData, TraceViewTokenData},
};
use husky_value_protocol::presentation::ValuePresentation;
use std::path::Path;
use ui::visual::cache::VisualUiCache;

pub(crate) struct TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceDocSettings,
{
    current_dir: &'a Path,
    trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
    generic_figure: &'a TraceProtocol::Figure,
    action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
    settings: &'a mut Settings,
    visual_ui_cache: &'a mut ui::visual::cache::VisualUiCache<Ui>,
    caryatid_ui_buffer: &'a mut <TraceProtocol::Caryatid as IsCaryatid>::UiBuffer,
    glyph_width: f32,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn new(
        current_dir: &'a Path,
        trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
        action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
        settings: &'a mut Settings,
        figure_ui_cache: &'a mut VisualUiCache<Ui>,
        caryatid_ui_buffer: &'a mut <TraceProtocol::Caryatid as IsCaryatid>::UiBuffer,
        ui: &mut egui::Ui,
    ) -> Self {
        let glyph_width =
            ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
        Self {
            current_dir,
            trace_synchrotron,
            generic_figure: trace_synchrotron.figure(),
            action_buffer,
            settings,
            glyph_width,
            caryatid_ui_buffer,
            visual_ui_cache: figure_ui_cache,
        }
    }
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceDocSettings,
{
    fn add_action(&mut self, action: TraceViewAction<TraceProtocol>)
    where
        TraceProtocol: IsTraceProtocol,
    {
        self.action_buffer.push(action)
    }
}
