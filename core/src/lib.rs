pub mod model;
pub mod parser;

use std::io::BufReader;
use parser::{filter_nodes_on_ways, parse_ways};
use wasm_bindgen::prelude::*;
use crate::model::Node;
use crate::parser::parse_nodes;

#[wasm_bindgen(start)]
pub fn setup() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse_osm_roads(xml: &str) -> Result<Box<[Node]>,JsValue>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)
        .map_err(|e| JsValue::from_str(&format!("node parse error: {}", e)))?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let all_ways = parse_ways(reader2)
        .map_err(|e| JsValue::from_str(&format!("way parse error: {}", e)))?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes.into_boxed_slice())
}


