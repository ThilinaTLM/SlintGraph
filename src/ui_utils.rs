use std::rc::Rc;

use slint::{Color, VecModel};

use crate::{graph::{Graph, Node}, xml_utils::{Action, Process}};

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
        let nodes: Vec<UiNodeData> = value.nodes.iter().map(UiNodeData::from).collect();
        let edges: Vec<UiEdgeData> = value.edges.iter().map(|edge| {
            let source_node = value.find_node(&edge.source).unwrap();
            let target_node = value.find_node(&edge.target).unwrap();
            let source_index: i32 = value.nodes.iter().position(|n| n.id == edge.source).unwrap().try_into().unwrap();
            let target_index: i32 = value.nodes.iter().position(|n| n.id == edge.target).unwrap().try_into().unwrap();

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

pub trait UiProcessExt {
    fn get_all_ui_actions(&self) -> Vec<UiAction>;
    fn get_ui_links(&self, actions: &Vec<Action>) -> Vec<UiLink>;
}

impl UiProcessExt for Process {
    fn get_all_ui_actions(&self) -> Vec<UiAction> {
        self.get_all_actions().iter().enumerate().map(|(index, action)| {

            let ui_actions: Vec<slint::SharedString> = action.meta_data.get_inputs_as_strings().into_iter().map(|s| s.into()).collect();
            let ui_outputs: Vec<slint::SharedString> = action.meta_data.get_outputs_as_strings().into_iter().map(|s| s.into()).collect();
            let ui_outcomes: Vec<slint::SharedString> = action.meta_data.get_outcomes_as_strings().into_iter().map(|s| s.into()).collect();

            let inputs_model = Rc::new(VecModel::from(ui_actions));
            let outputs_model = Rc::new(VecModel::from(ui_outputs));
            let outcomes_model = Rc::new(VecModel::from(ui_outcomes));

            UiAction {
                id: action.action_id.clone().into(),
                name: action.name.clone().into(),
                x: action.ui_hints.get_xloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                y: action.ui_hints.get_yloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                inputs: inputs_model.into(),
                outputs: outputs_model.into(),
                outcomes: outcomes_model.into(),
            }
        }).collect()
    }

    fn get_ui_links(&self, actions: &Vec<Action>) -> Vec<UiLink> {
        todo!()
    }
}

pub struct ProcessUiAdapter {
    pub process: Process,
    pub ui_actions: Vec<UiAction>,
    pub ui_links: Vec<UiLink>,
}

fn find_ui_action_index_by_id(actions: &Vec<UiAction>, id: &str) -> Option<usize> {
    actions.iter().enumerate().find_map(|(index, action)| {
        if action.id == id {
            Some(index)
        } else {
            None
        }
    })
}

impl ProcessUiAdapter {
    pub fn new(process: Process) -> Self {
        let all_actions = process.get_all_actions();

        let ui_actions: Vec<UiAction> = all_actions.iter().map(|action| {
            let ui_actions: Vec<slint::SharedString> = action.meta_data.get_inputs_as_strings().into_iter().map(|s| s.into()).collect();
            let ui_outputs: Vec<slint::SharedString> = action.meta_data.get_outputs_as_strings().into_iter().map(|s| s.into()).collect();
            let ui_outcomes: Vec<slint::SharedString> = action.meta_data.get_outcomes_as_strings().into_iter().map(|s| s.into()).collect();
            let inputs_model = Rc::new(VecModel::from(ui_actions));
            let outputs_model = Rc::new(VecModel::from(ui_outputs));
            let outcomes_model = Rc::new(VecModel::from(ui_outcomes));
            UiAction {
                id: action.action_id.clone().into(),
                name: action.name.clone().into(),
                x: action.ui_hints.get_xloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                y: action.ui_hints.get_yloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                inputs: inputs_model.into(),
                outputs: outputs_model.into(),
                outcomes: outcomes_model.into(),
            }
        }).collect();

        let mut ui_links = Vec::new();
        for action in all_actions {
            let source_action_index = find_ui_action_index_by_id(&ui_actions, &action.action_id);

            for outcome_link in &action.outcome_links {
                let action_id = &outcome_link.to_action_id;
                if (!action_id.is_empty()) {
                    let target_action_index = find_ui_action_index_by_id(&ui_actions, &action_id);
                    if let (Some(source_action_index), Some(target_action_index)) = (source_action_index, target_action_index) {
                        ui_links.push(UiLink {
                            id: outcome_link.link_id.clone().into(),
                            source_id: action.action_id.clone().into(),
                            source_type: UiElementType::Action,
                            source_index: source_action_index.try_into().expect("Index should fit into UiLink source_index type"),
                            target_id: action_id.clone().into(),
                            target_type: UiElementType::Action,
                            target_index: target_action_index.try_into().expect("Index should fit into UiLink target_index type"),
                        });
                    }
                }
            }
        }

        Self {
            process,
            ui_actions,
            ui_links
        }
    }
}