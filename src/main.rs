#![allow(unused)]

mod graph;
mod ui_utils;
mod xml_utils;

use std::{path::PrefixComponent, rc::Rc};
use graph::{Edge, Graph, Node};
use slint::{Color, ComponentHandle, Model, VecModel};
use ui_utils::{color_from_hex, AppState, SlintDemoWindow, UiDimention, UiEdgeData, UiGraph, UiNodeData, UiProcessExt};
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
        app_state.on_update_node(move |node| {
            controller.on_update_node(&node);
        });
    }

    fn on_update_node(&self, node: &UiNodeData) {
        println!("Node updated: {:?}", node);
        let ui = self.ui_weak.upgrade().unwrap();
        let graph_state = ui.global::<AppState>();
        
        // update node
        let nodes_model = graph_state.get_nodes();
        let nodes = nodes_model.iter().map(|n| {
            if n.id == node.id {
                node.clone()
            } else {
                n.clone()
            }
        }).collect::<Vec<_>>();
        let nodes_model: Rc<VecModel<UiNodeData>> = Rc::new(VecModel::from(nodes));

        // update edges
        let edges_model = graph_state.get_edges();
        let edges = edges_model.iter().map(|edge| {
            if edge.source == node.id {
                UiEdgeData {
                    id: edge.id.into(),
                    source: edge.source.into(),
                    target: edge.target.into(),
                    source_dim: UiDimention {
                        x: node.x,
                        y: node.y,
                        width: node.width,
                        height: node.height,
                    },
                    target_dim: edge.target_dim,
                    source_index: edge.source_index,
                    target_index: edge.target_index,
                }
            } else if edge.target == node.id {
                UiEdgeData {
                    id: edge.id.into(),
                    source: edge.source.into(),
                    target: edge.target.into(),
                    source_dim: edge.source_dim,
                    target_dim: UiDimention {
                        x: node.x,
                        y: node.y,
                        width: node.width,
                        height: node.height,
                    },
                    source_index: edge.source_index,
                    target_index: edge.target_index,
                }
            } else {
                edge.clone()
            }
        }).collect::<Vec<_>>();
        let edges_model = Rc::new(VecModel::from(edges));

        // update graph state
        graph_state.set_nodes(nodes_model.into());
        graph_state.set_edges(edges_model.into());

        // save
        graph_state.invoke_save();
    }

    fn load_data_from_file(&self, path: &str) {
        let process = Process::from_xml_file(path).unwrap();
        let ui_actions = process.get_all_ui_actions();

        // process ui actions
        let (max_x, max_y) = ui_actions.iter().fold((0f32, 0f32), |(max_x, max_y), action| {
            (f32::max(max_x, action.x), f32::max(max_y, action.y))
        });
        let ui_actions_model = Rc::new(VecModel::from(ui_actions));

        // process ui action links
        
        let ui = self.ui_weak.upgrade().unwrap();
        let app_state = ui.global::<AppState>();
        app_state.set_actions(ui_actions_model.into());
        app_state.set_viewport_width(max_x);
        app_state.set_viewport_height(max_y);
    }

    fn save_data_to_file(&self, path: &str) {
        let ui = self.ui_weak.upgrade().unwrap();
        let graph_state = ui.global::<AppState>();

        let nodes = graph_state.get_nodes();
        let edges = graph_state.get_edges();

        let graph_nodes = nodes.iter().map(|node| node.clone().into()).collect();
        let graph_edges = edges.iter().map(|edge| Edge {
            id: edge.id.into(),
            source: edge.source.into(),
            target: edge.target.into(),
        }).collect();

        let graph = Graph::from_nodes_and_edges(graph_nodes, graph_edges);
        graph.save_to_xml(&path).unwrap();
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = UiController::new("data/enactor-1-1.0.xml".to_string());
    ui.run();
    Ok(())
}