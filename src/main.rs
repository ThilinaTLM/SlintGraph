#![allow(unused)]

mod graph;
mod utils;

use std::rc::Rc;
use graph::{Edge, Graph, Node};
use slint::{Color, ComponentHandle, Model, VecModel};
use utils::{color_from_hex, AppState, SlintDemoWindow, UiDimention, UiEdgeData, UiGraph, UiNodeData};

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
        let graph_state = ui.global::<AppState>();

        let controller = self.clone();
        graph_state.on_save(move || {
            controller.save_data_to_file(&controller.file_path);
        });

        let controller = self.clone();
        graph_state.on_update_node(move |node| {
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
        let graph = Graph::from_xml(path).unwrap();
        let ui_graph = UiGraph::from(&graph);

        let ui_nodes_model = Rc::new(slint::VecModel::from(ui_graph.nodes));
        let ui_edges_model = Rc::new(slint::VecModel::from(ui_graph.edges));

        let ui = self.ui_weak.upgrade().unwrap();
        let graph_state = ui.global::<AppState>();
        graph_state.set_nodes(ui_nodes_model.into());
        graph_state.set_edges(ui_edges_model.into());
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
    let ui = UiController::new("data/graph-1.xml".to_string());
    ui.run();
    Ok(())
}