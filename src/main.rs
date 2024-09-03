#![allow(unused)]

mod ui_utils;
mod xml_utils;

use slint::{Color, ComponentHandle, Model, VecModel};
use std::{path::PrefixComponent, rc::Rc};
use ui_utils::{AppState, SlintDemoWindow, UiDimention, UiProcessAdapter};
use xml_utils::Process;

struct UiController {
    file_path: String,
    ui: SlintDemoWindow,
    ui_weak: slint::Weak<SlintDemoWindow>,
}

impl UiController {
    fn new(file_path: String) -> Rc<Self> {
        let ui = SlintDemoWindow::new().unwrap();
        let controller = Rc::new(Self {
            file_path,
            ui_weak: ui.as_weak(),
            ui,
        });

        controller.setup_handlers();
        controller.load_data_from_file(&controller.file_path);

        controller
    }

    fn run(self: &Rc<Self>) {
        let ui = self.ui_weak.upgrade().unwrap();
        ui.run().unwrap();
    }

    fn setup_handlers(self: &Rc<Self>) {
        let ui = self.ui_weak.upgrade().unwrap();
        let app_state = ui.global::<AppState>();

        let controller = self.clone();
        app_state.on_save(move || {
            controller.save_data_to_file(&controller.file_path);
        });

        let controller = self.clone();
    }

    fn load_data_from_file(&self, path: &str) {
        let process = Process::from_xml_file(path).unwrap();
        let process_ui_adapter = UiProcessAdapter::new(process);

        let ui_nodes = process_ui_adapter.ui_nodes;
        let ui_links = process_ui_adapter.ui_links;

        // process ui actions
        let (max_x, max_y) = ui_nodes
            .iter()
            .fold((0f32, 0f32), |(max_x, max_y), action| {
                (f32::max(max_x, action.x), f32::max(max_y, action.y))
            });
        let ui_nodes_model = Rc::new(VecModel::from(ui_nodes));

        // process ui action links
        let ui_links_model = Rc::new(VecModel::from(ui_links));
        let ui = self.ui_weak.upgrade().unwrap();
        let app_state = ui.global::<AppState>();
        app_state.set_viewport_width(max_x);
        app_state.set_viewport_height(max_y);
        app_state.set_nodes(ui_nodes_model.into());
        app_state.set_links(ui_links_model.into());
    }

    fn save_data_to_file(&self, path: &str) {
        todo!();
    }
}

fn main() {
    let ui = UiController::new("data/enactor-1-1.0.xml".to_string());
    ui.run();
}
