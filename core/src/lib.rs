pub mod model;
pub mod parser;

use std::io::BufReader;
use crate::model::{OsmError, Node, Way};
use crate::parser::{parse_nodes, parse_ways, filter_nodes_on_ways, parse_ways_with_tags};


pub fn parse_osm_ways(xml: &str) -> Result<Vec<Node>,OsmError>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let all_ways = parse_ways(reader2)?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes)
}

pub fn parse_osm_ways_with_tags_nodes(xml: &str, tags: Vec<String>) -> Result<Vec<Node>,OsmError>  {
    let reader = BufReader::new(xml.as_bytes());
    let all_nodes = parse_nodes(reader)?;
    
    let reader2 = BufReader::new(xml.as_bytes());
    let tag_slices: Vec<&str> = tags.iter().map(String::as_str).collect();
    let all_ways = parse_ways_with_tags(&tag_slices, reader2)?;

    let road_nodes = filter_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes)
}

pub fn parse_osm_ways_with_tags_ways(xml: &str, tags: Vec<String>) -> Result<Vec<Way>,OsmError>  {
    let reader = BufReader::new(xml.as_bytes());
    let tag_slices: Vec<&str> = tags.iter().map(String::as_str).collect();
    let all_ways = parse_ways_with_tags(&tag_slices, reader)?;
    Ok(all_ways)
}


