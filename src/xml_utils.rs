
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Documents {
    #[serde(rename = "document")]
    documents: Vec<Document>,
}

#[derive(Debug, Deserialize)]
struct Document {
    index: String,
    source: String,
    document_content: DocumentContent,
}

#[derive(Debug, Deserialize)]
struct DocumentContent {
    #[serde(rename = "core:process")]
    process: Process,
}

#[derive(Debug, Deserialize)]
struct Process {
    #[serde(rename = "core:processID")]
    process_id: String,
    #[serde(rename = "core:version")]
    version: String,
    #[serde(rename = "core:name")]
    name: String,
}


fn parse_xml(file_path: &str) -> Result<DocumentContent, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let file = std::io::BufReader::new(file);
    let document: DocumentContent = quick_xml::de::from_reader(file)?;
    Ok(document)
}


// write unit test for parse_xml function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xml() {
        let file_path = "/home/tlm/Projects/SlintGraph/data/enactor-1-1.0.xml";
        let document = parse_xml(file_path).unwrap();
        assert_eq!(document.process.process_id, "1");
        assert_eq!(document.process.version, "1.0");
        assert_eq!(document.process.name, "Test Process");
    }
}