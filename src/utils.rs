use slint::Color;

use crate::graph::{Graph, Node};

slint::include_modules!();

pub fn color_from_hex(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
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
        }
    }
}

pub struct UiGraph {
    pub nodes: Vec<UiNodeData>,
    pub edges: Vec<UiEdgeData>,
}

impl From<&Graph> for UiGraph {

    fn from(value: &Graph) -> Self {
        let nodes: Vec<UiNodeData> = value.nodes.node.iter().map(UiNodeData::from).collect();
        let edges: Vec<UiEdgeData> = value.edges.edge.iter().map(|edge| {
            let source_node = value.find_node(&edge.source).unwrap();
            let target_node = value.find_node(&edge.target).unwrap();
            let source_index: i32 = value.nodes.node.iter().position(|n| n.id == edge.source).unwrap().try_into().unwrap();
            let target_index: i32 = value.nodes.node.iter().position(|n| n.id == edge.target).unwrap().try_into().unwrap();

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
                source_index: source_index,
                target_index: target_index,
            }
        }).collect();
        UiGraph {
            nodes,
            edges,
        }
    }
}