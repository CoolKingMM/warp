use std::path::PathBuf;

use warpui::elements::{Container, Empty};
use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

use crate::code::active_file::ActiveFileModel;
use crate::coding_panel_enablement_state::CodingPanelEnablementState;

#[derive(Default)]
pub struct FileTreeView;

impl FileTreeView {
    pub fn new(_: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_root_directories(&mut self, _: Vec<PathBuf>, _: &mut ViewContext<Self>) {}

    pub fn set_has_terminal_session(&mut self, _: bool, _: &mut ViewContext<Self>) {}

    pub fn set_active_file_model(
        &mut self,
        _: warpui::ModelHandle<ActiveFileModel>,
        _: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_is_active(&mut self, _: bool, _: &mut ViewContext<Self>) {}

    pub fn auto_expand_to_most_recent_directory(&mut self, _: &mut ViewContext<Self>) {}

    pub fn set_enablement_state(
        &mut self,
        _: CodingPanelEnablementState,
        _: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_left_panel_focused(&mut self, _: &mut ViewContext<Self>) {}
}

pub enum FileTreeEvent {}

impl Entity for FileTreeView {
    type Event = FileTreeEvent;
}

impl View for FileTreeView {
    fn ui_name() -> &'static str {
        "FilePicker"
    }

    fn render(&self, _: &AppContext) -> Box<dyn Element> {
        Container::new(Empty::new().finish()).finish()
    }
}

impl TypedActionView for FileTreeView {
    type Action = ();

    fn handle_action(&mut self, _: &Self::Action, _: &mut ViewContext<Self>) {}
}
