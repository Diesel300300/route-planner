use std::fs::File;
use std::io::BufReader;
use route_parser::parser::parse_nodes;

#[test]
fn run_on_sample_file() {
    let reader = BufReader::new(File::open("../frontend/public/data/map").unwrap());
    let nodes = route_parser::parse_osm_roads(reader).expect("parse_nodes failed");

}
