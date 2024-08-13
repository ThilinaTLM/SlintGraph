mod graph;

use std::rc::Rc;
use graph::{Edge, Graph, Node};
use slint::{Color, Model, VecModel};

slint::include_modules!();

fn color_from_hex(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    let rgb = u32::from_str_radix(hex, 16)?;
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    Ok(Color::from_rgb_u8(r, g, b))
}

impl From<&Node> for UiNodeData {
    fn from(node: &Node) -> Self {
        UiNodeData {
            id: node.id.clone().into(),
            label: node.label.clone().into(),
            x: node.x as f32,
            y: node.y as f32,
            width: node.width as f32,
            height: node.height as f32,
            background: color_from_hex(&node.background).unwrap_or(Color::from_rgb_u8(0, 0, 0)),
        }
    }
}

impl Into<Node> for UiNodeData {
    fn into(self) -> Node {
        Node {
            id: self.id.to_string(),
            label: self.label.to_string(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            background: format!("#{:02X}{:02X}{:02X}", self.background.red(), self.background.green(), self.background.blue()),
        }
    }
}

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
        let graph_state = ui.global::<GraphState>();

        let controller = self.clone();
        graph_state.on_changed(move || {
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
        let graph_state = ui.global::<GraphState>();
        let nodes_model = graph_state.get_nodes();
        let nodes = nodes_model.iter().map(|n| {
            if n.id == node.id {
                node.clone()
            } else {
                n.clone()
            }
        }).collect::<Vec<_>>();
        let nodes_model = Rc::new(VecModel::from(nodes));
        graph_state.set_nodes(nodes_model.into());
        graph_state.invoke_changed();
    }

    fn load_data_from_file(&self, path: &str) {
        let graph = Graph::from_xml(path).unwrap();
        let ui_nodes: Vec<UiNodeData> = graph.nodes.node.iter().map(UiNodeData::from).collect();
        let ui_edges: Vec<UiEdgeData> = graph.edges.edge.iter().map(|edge| {
            let source_node = graph.find_node(&edge.source).unwrap();
            let target_node = graph.find_node(&edge.target).unwrap();
            UiEdgeData {
                id: edge.id.clone().into(),
                source: edge.source.clone().into(),
                target: edge.target.clone().into(),
                source_dim: UiDimention {
                    x: source_node.x,
                    y: source_node.y,
                    width: source_node.width,
                    height: source_node.height,
                },
                target_dim: UiDimention {
                    x: target_node.x,
                    y: target_node.y,
                    width: target_node.width,
                    height: target_node.height,
                },
            }
        }).collect();

        let ui_nodes_model = Rc::new(slint::VecModel::from(ui_nodes));
        let ui_edges_model = Rc::new(slint::VecModel::from(ui_edges));

        let ui = self.ui_weak.upgrade().unwrap();
        let graph_state = ui.global::<GraphState>();
        graph_state.set_nodes(ui_nodes_model.into());
        graph_state.set_edges(ui_edges_model.into());
    }

    fn save_data_to_file(&self, path: &str) {
        let ui = self.ui_weak.upgrade().unwrap();
        let graph_state = ui.global::<GraphState>();

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
    let ui = UiController::new("data/graph-2.xml".to_string());
    ui.run();
    Ok(())
}