pub mod model;
pub mod parser;
pub mod graph;
pub mod builder;

use std::{fs::File, io::BufReader};
use crate::model::{OsmError, Way};
use crate::parser::{parse_nodes, get_nodes_on_ways, parse_ways_with_tags};
use crate::graph::Graph;
use crate::builder::GraphBuilder;


pub fn parse_osm_ways(file_path: &str, tags: &[&str]) -> Result<Vec<Way>,OsmError>  {

    let file1 = File::open(file_path).expect(String::as_str(&format!("failed to open {}", file_path)));
    let reader1 = BufReader::new(file1);
    let all_nodes = parse_nodes(reader1)?;
    
    let file2 = File::open(file_path).expect(String::as_str(&format!("failed to open {}", file_path)));
    let reader2 = BufReader::new(file2);
    let all_ways = parse_ways_with_tags(tags ,reader2)?;

    let road_nodes = get_nodes_on_ways(all_nodes, &all_ways);
    Ok(road_nodes)
}

pub fn create_graph(file_path: &str,tags: &[&str]) -> Result<Graph, OsmError> {
    let ways = parse_osm_ways(file_path, tags)?;
    let mut graph_builder = GraphBuilder::new();
    for way in ways.iter() {
        graph_builder.add_way(way);
    }

    let graph = graph_builder.build();
    Ok(graph)
}

