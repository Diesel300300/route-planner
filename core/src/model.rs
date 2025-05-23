use quick_xml::Error as xmlError;
use quick_xml::events::attributes::AttrError;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum OsmError {
    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] xmlError),
    #[error("Attribute unwrap error: {0}")]
    AttributeParseError(#[from] AttrError),
}

#[derive(Debug, Clone, Serialize)]
pub struct Path {
    pub id: Uuid, // add id to have difference between paths in frontend
    pub distance: f64,
    pub nodes: Vec<Node>
}

impl Path {

    pub fn new(nodes: Vec<Node>, distance: f64) -> Path {
        Path { id: Uuid::new_v4(), distance, nodes }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct EdgeData {
    pub way_id: u64,
    pub length_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Neighbor {
    pub osm_id: u64,
    pub node_index: usize,
    pub edge_data: EdgeData,
}


#[derive(Debug, Clone,Copy, Serialize)]
pub struct Node {
    id: u64,
    lat: f64, 
    lon: f64,
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
    pub nodes: Vec<Node>,
    
    #[serde(skip_serializing)]
    pub node_refs: Vec<u64> // vec containing the OSM node ids
}

impl Way {
    pub fn new(id: u64, node_refs: Vec<u64>, nodes: Vec<Node>) -> Self {
        Way { id, node_refs, nodes }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn node_refs(&self) -> Vec<u64> {
        self.node_refs.clone()
    }
    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

}

