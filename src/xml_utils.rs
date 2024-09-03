#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(rename = "@xmlns:core", default)]
    xmlns_core: String,

    #[serde(rename = "@xmlns:ns5", default)]
    xmlns_ns5: String,

    #[serde(rename = "@xmlns:ns7", default)]
    xmlns_ns7: String,

    #[serde(rename = "@xmlns:retail", default)]
    xmlns_retail: String,

    #[serde(rename = "@xmlns:sref", default)]
    xmlns_sref: String,

    #[serde(rename = "@xmlns:tools", default)]
    xmlns_tools: String,

    #[serde(rename = "processID", default)]
    process_id: String,

    #[serde(default)]
    version: String,

    #[serde(default)]
    name: Option<String>,

    #[serde(rename = "defaultMessageBaseName", default)]
    default_message_base_name: Option<String>,

    #[serde(rename = "firstStateID", default)]
    first_state_id: String,

    #[serde(rename = "className", default)]
    class_name: Option<String>,

    #[serde(rename = "uiHints", default)]
    ui_hints: UiHints,
    
    #[serde(rename = "metaData", default)]
    meta_data: MetaData,

    #[serde(rename = "state", default)]
    pub states: Vec<State>,
    
    #[serde(rename = "action", default)]
    pub actions: Vec<Action>,

    #[serde(rename = "endProcessAction", default)]
    pub end_process_actions: Vec<Action>,

    #[serde(rename = "executeProcessAction", default)]
    pub execute_process_actions: Vec<Action>,

    #[serde(rename = "assignAction", default)]
    pub assign_actions: Vec<Action>,
}

impl Process {
    
    pub fn from_xml_string(xml_string: &str) -> Result<Process, Box<dyn std::error::Error>> {
        let document: Process = quick_xml::de::from_str(xml_string)?;
        Ok(document)
    }

    pub fn from_xml_file(file_path: &str) -> Result<Process, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let file = std::io::BufReader::new(file);
        let document: Process = quick_xml::de::from_reader(file)?;
        Ok(document)
    }

    pub fn to_xml_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let xml_string = quick_xml::se::to_string(self)?;
        Ok(xml_string)
    }

    pub fn to_xml_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut buffer);
        ser.indent(' ', 2);
        self.serialize(ser).unwrap();
        std::fs::write(file_path, buffer)?;
        Ok(())
    }

    pub fn get_all_actions(&self) -> Vec<&Action> {
        let mut all_actions = Vec::new();
        
        all_actions.extend(self.actions.iter());
        all_actions.extend(self.end_process_actions.iter());
        all_actions.extend(self.execute_process_actions.iter());
        all_actions.extend(self.assign_actions.iter());
        
        all_actions
    }

    pub fn get_all_states(&self) -> Vec<&State> {
        self.states.iter().collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UiHints {
    #[serde(rename = "entry", default)]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    key: String,
    value: String,
}

const UI_HINT_ENTRY_KEY_STYLE: &str = "com.enactor.tools.editor.process.style";
const UI_HINT_ENTRY_KEY_XLOC: &str = "com.enactor.tools.editor.process.xloc";
const UI_HINT_ENTRY_KEY_YLOC: &str = "com.enactor.tools.editor.process.yloc";

impl UiHints {
    pub fn get_entry(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.key == key)
    }

    pub fn set_entry(&mut self, key: &str, value: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            entry.value = value.to_string();
        } else {
            self.entries.push(Entry {
                key: key.to_string(),
                value: value.to_string(),
            });
        }
    }

    pub fn get_style(&self) -> Option<&String> {
        self.get_entry(UI_HINT_ENTRY_KEY_STYLE).map(|entry| &entry.value)
    }

    pub fn get_xloc(&self) -> Option<&String> {
        self.get_entry(UI_HINT_ENTRY_KEY_XLOC).map(|entry| &entry.value)
    }

    pub fn get_yloc(&self) -> Option<&String> {
        self.get_entry(UI_HINT_ENTRY_KEY_YLOC).map(|entry| &entry.value)
    }

    pub fn set_style(&mut self, style: &str) {
        self.set_entry(UI_HINT_ENTRY_KEY_STYLE, style);
    }

    pub fn set_xloc(&mut self, xloc: &str) {
        self.set_entry(UI_HINT_ENTRY_KEY_XLOC, xloc);
    }

    pub fn set_yloc(&mut self, yloc: &str) {
        self.set_entry(UI_HINT_ENTRY_KEY_YLOC, yloc);
    }
}



#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MetaData {
    #[serde(rename = "stateDataTypes", default)]
    state_data_types: Option<StateDataTypes>,

    #[serde(rename = "inputs")]
    inputs: Option<Inputs>,

    #[serde(rename = "outputs")]
    outputs: Option<Outputs>,

    #[serde(rename = "outcomes")]
    outcomes: Option<Outcomes>,

    #[serde(rename = "handledEvents")]
    handled_events: Option<HandledEvents>,
}

impl MetaData {
    pub fn get_inputs_as_strings(&self) -> Vec<String> {
        self.inputs
            .as_ref()
            .map(|inputs| {
                inputs
                    .inputs
                    .iter()
                    .map(|input| input.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_outputs_as_strings(&self) -> Vec<String> {
        self.outputs
            .as_ref()
            .map(|outputs| {
                outputs
                    .outputs
                    .iter()
                    .map(|output| output.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_outcomes_as_strings(&self) -> Vec<String> {
        self.outcomes
            .as_ref()
            .map(|outcomes| {
                outcomes
                    .outcomes
                    .iter()
                    .map(|outcome| outcome.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_state_data_types_as_strings(&self) -> Vec<String> {
        self.state_data_types
            .as_ref()
            .map(|state_data_types| {
                state_data_types
                    .state_data_types
                    .iter()
                    .map(|state_data_type| state_data_type.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_handled_events_as_strings(&self) -> Vec<String> {
        self.handled_events
            .as_ref()
            .map(|handled_events| {
                handled_events
                    .events
                    .iter()
                    .map(|event| event.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Inputs {
    #[serde(rename = "input", default)]
    inputs: Vec<Input>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    #[serde(rename = "@required", default)]
    required: bool,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "interfaceName")]
    interface_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct StateDataTypes {
    #[serde(rename = "stateDataType", default)]
    state_data_types: Vec<StateDataType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateDataType {
    #[serde(rename = "@required", default)]
    required: bool,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "interfaceName")]
    interface_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "defaultNextStateID")]
    default_next_state_id: Option<String>,

    #[serde(rename = "actionID")]
    pub action_id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "className")]
    class_name: String,

    #[serde(rename = "uiHints")]
    pub ui_hints: UiHints,

    #[serde(rename = "metaData")]
    pub meta_data: MetaData,

    #[serde(rename = "outcomeLink", default)]
    pub outcome_links: Vec<OutcomeLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Outputs {
    #[serde(rename = "output")]
    outputs: Vec<Output>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "interfaceName")]
    interface_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Outcomes {
    #[serde(rename = "outcome")]
    outcomes: Vec<Outcome>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Outcome {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutcomeLink {
    #[serde(rename = "linkID")]
    pub link_id: String,

    #[serde(rename = "toStateID")]
    pub to_state_id: Option<String>,

    #[serde(rename = "toActionID")]
    pub to_action_id: Option<String>,

    #[serde(rename = "condition")]
    pub condition: Option<String>,

    #[serde(rename = "outcome")]
    pub outcome: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "stateID")]
    pub state_id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "className")]
    pub class_name: String,

    #[serde(rename = "defaultNextStateID")]
    default_next_state_id: Option<String>,

    #[serde(rename = "uiHints")]
    pub ui_hints: UiHints,

    #[serde(rename = "metaData")]
    pub meta_data: MetaData,

    #[serde(rename = "typeId")]
    pub type_id: String,

    #[serde(rename = "singleInstance")]
    pub single_instance: bool,

    #[serde(rename = "respondToViewEvents")]
    pub respond_to_view_events: bool,

    #[serde(rename = "actionInputMappings")]
    pub action_input_mappings: ActionMappings,

    #[serde(rename = "actionOutputMappings")]
    pub action_output_mappings: ActionMappings,

    #[serde(rename = "eventLink")]
    pub event_links: Vec<EventLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandledEvents {
    #[serde(rename = "handledEvent")]
    pub events: Vec<HandledEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandledEvent {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionMappings {
    #[serde(rename = "actionMapping")]
    mappings: Vec<ActionMapping>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionMapping {
    #[serde(rename = "actionID")]
    action_id: String,

    #[serde(rename = "mappings")]
    mappings: Mappings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mappings {
    #[serde(rename = "processMappings")]
    process_mappings: ProcessMappings,

    #[serde(rename = "stateMappings")]
    state_mappings: StateMappings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessMappings {
    #[serde(rename = "mappings", default)]
    mappings: Vec<Mapping>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateMappings {
    #[serde(rename = "mappings")]
    mappings: Vec<Mapping>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(rename = "sourceDataType")]
    source_data_type: Option<DataType>,

    #[serde(rename = "targetDataType")]
    target_data_type: Option<DataType>,

    #[serde(rename = "expression", default)]
    expression: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataType {
    #[serde(rename = "@required", default)]
    required: bool,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "interfaceName")]
    interface_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventLink {
    #[serde(rename = "linkID")]
    pub link_id: String,

    #[serde(rename = "toStateID")]
    pub to_state_id: Option<String>,

    #[serde(rename = "toActionID")]
    pub to_action_id: Option<String>,

    #[serde(rename = "condition")]
    pub condition: Option<String>,

    #[serde(rename = "event")]
    pub event: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let file_path = "/home/tlm/Projects/SlintGraph/data/enactor-1-1.0.xml";
        let process = Process::from_xml_file(file_path).unwrap();
        println!("{:#?}", process);
    }
}