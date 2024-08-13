use quick_xml;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "graph")]
pub struct Graph {
    #[serde(rename = "node")]
    pub nodes: Vec<Node>,
    #[serde(rename = "edge")]
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn from_nodes_and_edges(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph {
            nodes: nodes,
            edges: edges,
        }
    }

    pub fn from_xml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(file_path)?;
        let file = BufReader::new(file);
        let graph: Graph = quick_xml::de::from_reader(file)?;
        Ok(graph)
    }

    pub fn find_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn save_to_xml(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut buffer);
        ser.indent(' ', 2);
        self.serialize(ser).unwrap();
        fs::write(file_path, buffer)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}