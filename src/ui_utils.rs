use std::rc::Rc;

use slint::{Color, VecModel};

use crate::xml_utils::{Action, Process};

slint::include_modules!();

pub struct UiProcessAdapter {
    pub process: Process,
    pub ui_nodes: Vec<UiNode>,
    pub ui_links: Vec<UiLink>,
}

fn find_ui_action_index_by_id(nodes: &Vec<UiNode>, id: &str) -> Option<usize> {
    nodes.iter().enumerate().find_map(|(index, action)| {
        if action.id == id {
            Some(index)
        } else {
            None
        }
    })
}

fn get_simple_name(qualified_name: &str) -> String {
    qualified_name
        .split(".")
        .last()
        .unwrap_or(qualified_name)
        .to_string()
}

impl UiProcessAdapter {
    pub fn new(process: Process) -> Self {
        let all_actions = process.get_all_actions();

        let ui_actions: Vec<UiNode> = all_actions.iter().map(|action| {
            let ui_actions: Vec<UiSectionItem> = action.meta_data.get_inputs_as_strings().into_iter().map(|s| {
                UiSectionItem {
                    name: s.clone().into(),
                    simple_name: get_simple_name(&s).into(),
                    required: false,
                    unused: false,
                }
            }).collect();
            let ui_outputs: Vec<UiSectionItem> = action.meta_data.get_outputs_as_strings().into_iter().map(|s| {
                UiSectionItem {
                    name: s.clone().into(),
                    simple_name: get_simple_name(&s).into(),
                    required: false,
                    unused: false,
                }
            }).collect();
            let ui_outcomes: Vec<UiSectionItem> = action.meta_data.get_outcomes_as_strings().into_iter().map(|s| {
                UiSectionItem {
                    name: s.clone().into(),
                    simple_name: get_simple_name(&s).into(),
                    required: false,
                    unused: false,
                }
            }).collect();

            let inputs_model = Rc::new(VecModel::from(ui_actions));
            let outputs_model = Rc::new(VecModel::from(ui_outputs));
            let outcomes_model = Rc::new(VecModel::from(ui_outcomes));
            UiNode {
                id: action.action_id.clone().into(),
                name: action.name.clone().into(),
                class: UiNodeClass::Action,
                x: action.ui_hints.get_xloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                y: action.ui_hints.get_yloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                inputs: inputs_model.into(),
                outputs: outputs_model.into(),
                outcomes: outcomes_model.into(),
                states: Rc::new(VecModel::from(Vec::new())).into(),
                events: Rc::new(VecModel::from(Vec::new())).into(),
            }
        }).collect();

        let all_states = process.get_all_states();
        let ui_states: Vec<UiNode> = all_states.iter().map(|state| {
            let states: Vec<UiSectionItem> = state.meta_data.get_state_data_types_as_strings().into_iter().map(|s| {
                UiSectionItem {
                    name: s.clone().into(),
                    simple_name: get_simple_name(&s).into(),
                    required: false,
                    unused: false,
                }
            }).collect();

            let ui_states_model = Rc::new(VecModel::from(states));

            UiNode {
                id: state.state_id.clone().into(),
                name: state.name.clone().into(),
                class: UiNodeClass::State,
                x: state.ui_hints.get_xloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                y: state.ui_hints.get_yloc().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
                inputs: Rc::new(VecModel::from(Vec::new())).into(),
                outputs: Rc::new(VecModel::from(Vec::new())).into(),
                outcomes: Rc::new(VecModel::from(Vec::new())).into(),
                states: ui_states_model.into(),
                events: Rc::new(VecModel::from(Vec::new())).into(),
            }

        }).collect();

        let ui_nodes: Vec<UiNode> = ui_actions.iter().chain(ui_states.iter()).cloned().collect();

        let mut ui_links = Vec::new();
        for action in all_actions {
            let source_action_index = find_ui_action_index_by_id(&ui_actions, &action.action_id);

            for (outcome_index, outcome_link) in action.outcome_links.iter().enumerate() {
                let action_id = &outcome_link.to_action_id;
                if (!action_id.is_empty()) {
                    let target_action_index = find_ui_action_index_by_id(&ui_actions, &action_id);
                    if let (Some(source_action_index), Some(target_action_index)) = (source_action_index, target_action_index) {
                        ui_links.push(UiLink {
                            id: outcome_link.link_id.clone().into(),
                            source_id: action.action_id.clone().into(),
                            source_type: UiNodeClass::Action,
                            source_index: source_action_index.try_into().expect("Index should fit into UiLink source_index type"),
                            source_outcome_index: outcome_index.try_into().expect("Index should fit into UiLink source_outcome_index type"),
                            target_id: action_id.clone().into(),
                            target_type: UiNodeClass::Action,
                            target_index: target_action_index.try_into().expect("Index should fit into UiLink target_index type"),
                        });
                    }
                }
            }
        }

        Self {
            process,
            ui_nodes,
            ui_links
        }
    }
}