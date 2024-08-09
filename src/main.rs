
slint::include_modules!();

use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use slint::{Color, Model};

fn color_from_hex(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    let rgb = u32::from_str_radix(hex, 16)?;
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    Ok(Color::from_rgb_u8(r, g, b))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "graph")]
pub struct Graph {
    #[serde(rename = "nodes")]
    pub nodes: Nodes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    #[serde(rename = "node")]
    pub node: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub background: String,
}

impl From<Node> for UiNodeData {
    fn from(node: Node) -> Self {
        UiNodeData {
            id: node.id.into(),
            label: node.label.into(),
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
            x: self.x as u32,
            y: self.y as u32,
            width: self.width as u32,
            height: self.height as u32,
            background: format!("#{:02X}{:02X}{:02X}", self.background.red(), self.background.green(), self.background.blue()),
        }
    }
}

fn parse_xml(file_path: &str) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file);

    let graph: Graph = quick_xml::de::from_reader(file)?;
    Ok(graph.nodes.node)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let ui = SlintDemoWindow::new()?;
    let ui = std::rc::Rc::new(ui);
    
    let nodes = parse_xml("data/graph-2.xml")?;
    let nodes: Vec<UiNodeData> = nodes.into_iter().map(UiNodeData::from).collect();
    let nodes_model = std::rc::Rc::new(slint::VecModel::from(nodes));
    ui.set_nodes(nodes_model.into());

    let ui_handler = ui.clone();
    ui.on_node_data_change(move || {
        let nodes = ui_handler.get_nodes();
        let nodes: Vec<Node> = nodes.iter().map(|node| node.clone().into()).collect();
        let graph = Graph {
            nodes: Nodes {
                node: nodes,
            },
        };
        let content = quick_xml::se::to_string(&graph).unwrap();
        std::fs::write("data/graph-2.xml", content).unwrap();
    });

    ui.run().unwrap();

    Ok(())
}
