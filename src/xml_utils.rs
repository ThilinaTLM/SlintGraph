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
    
    #[serde(rename = "action", default)]
    actions: Vec<Action>,

    #[serde(rename = "endProcessAction", default)]
    end_process_actions: Vec<Action>,

    #[serde(rename = "executeProcessAction", default)]
    execute_process_actions: Vec<Action>,

    #[serde(rename = "assignAction", default)]
    assign_actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UiHints {
    #[serde(rename = "entry", default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MetaData {
    #[serde(rename = "inputs")]
    inputs: Option<Inputs>,

    #[serde(rename = "outputs")]
    outputs: Option<Outputs>,

    #[serde(rename = "outcomes")]
    outcomes: Option<Outcomes>,

    #[serde(rename = "stateDataTypes", default)]
    state_data_types: StateDataTypes,
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
    #[serde(rename = "@required")]
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
    action_id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "className")]
    class_name: String,

    #[serde(rename = "uiHints")]
    ui_hints: UiHints,

    #[serde(rename = "metaData")]
    meta_data: MetaData,

    #[serde(rename = "outcomeLink", default)]
    outcome_link: Vec<OutcomeLink>,
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
    link_id: String,

    #[serde(rename = "toStateID")]
    to_state_id: Option<String>,

    #[serde(rename = "toActionID")]
    to_action_id: String,

    #[serde(rename = "condition")]
    condition: Option<String>,

    #[serde(rename = "outcome")]
    outcome: String,
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