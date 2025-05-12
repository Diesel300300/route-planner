use quick_xml::Error as xmlError;
use quick_xml::events::attributes::AttrError;
use serde::Serialize;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum OsmError {
    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] xmlError),
    #[error("Attribute unwrap error: {0}")]
    AttributeParseError(#[from] AttrError),
}


#[derive(Debug, Clone)]
pub struct Node {
    id: u64,
    lat: f64, 
    lon: f64
}

impl Node {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl Node {
    pub fn new(id: u64, lat: f64, lon: f64) -> Self {
        Node { id, lat, lon }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Way {
    pub id: u64,
    pub node_refs: Vec<u64>
}

impl Way {
    pub fn new(id: u64, node_refs: Vec<u64>) -> Self {
        Way { id, node_refs }
    }

    pub fn node_refs(&self) -> Vec<u64> {
        // check if this is worth or i need to find something else
        self.node_refs.clone()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

