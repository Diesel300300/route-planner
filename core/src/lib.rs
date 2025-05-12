pub mod model;
pub mod parser;

use std::io::BufReader;
use wasm_bindgen::prelude::*;
use crate::model::Node;
use crate::parser::{parse_nodes, parse_ways, filter_nodes_on_ways, parse_ways_with_tag, parse_ways_with_tag_and_exclude_tag, parse_ways_with_tags};

#[wasm_bindgen(start)]
pub fn setup() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse_osm_ways(xml: &str) -> Result<Box<[Node]>,JsValue>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)
        .map_err(|e| JsValue::from_str(&format!("node parse error: {}", e)))?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let all_ways = parse_ways(reader2)
        .map_err(|e| JsValue::from_str(&format!("way parse error: {}", e)))?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes.into_boxed_slice())
}

#[wasm_bindgen]
pub fn parse_osm_ways_with_tag(xml: &str, tag: &str) -> Result<Box<[Node]>,JsValue>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)
        .map_err(|e| JsValue::from_str(&format!("node parse error: {}", e)))?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let all_ways = parse_ways_with_tag(tag, reader2)
        .map_err(|e| JsValue::from_str(&format!("way parse error: {}", e)))?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes.into_boxed_slice())
}

#[wasm_bindgen]
pub fn parse_osm_ways_with_tags_and_exclude(xml: &str, tag: &str, exclude_tag:&str) -> Result<Box<[Node]>,JsValue>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)
        .map_err(|e| JsValue::from_str(&format!("node parse error: {}", e)))?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let all_ways = parse_ways_with_tag_and_exclude_tag(tag, exclude_tag, reader2)
        .map_err(|e| JsValue::from_str(&format!("way parse error: {}", e)))?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes.into_boxed_slice())
}

#[wasm_bindgen]
pub fn parse_osm_ways_with_tags(xml: &str, tags: Vec<String>) -> Result<Box<[Node]>,JsValue>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)
        .map_err(|e| JsValue::from_str(&format!("node parse error: {}", e)))?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let tag_slices: Vec<&str> = tags.iter().map(String::as_str).collect();
    let all_ways = parse_ways_with_tags(&tag_slices, reader2)
        .map_err(|e| JsValue::from_str(&format!("way parse error: {}", e)))?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes.into_boxed_slice())
}


