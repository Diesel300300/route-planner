use core::str;
use std::collections::HashSet;
use std::panic;
use std::{
    collections::HashMap, 
    io::BufRead,
};
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::model::{ self, Node, OsmError, Way };



pub fn filter_nodes_on_ways(nodes: Vec<Node>, ways: &[Way]) -> Vec<Node> {
    let mut road_ids = HashSet::new();
    for way in ways {
        for &nid in &way.node_refs {
            road_ids.insert(nid);
        }
    }

    nodes.into_iter()
        .filter(|node| road_ids.contains(&node.id()))
        .collect()
}


// returns all the nodes
pub fn parse_ways<R: BufRead>(reader: R) -> Result<Vec<Way>,OsmError> {
    let mut xml = Reader::from_reader(reader);
    xml.trim_text(true);
    let mut buf = Vec::new();
    let mut ways = Vec::new();
    
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref way)) if way.name().0 == b"way" => {
                let mut node_refs: Vec<u64> = Vec::new();
                let mut way_id: u64 = 0;
                for attr in way.attributes() {
                    let attr = attr?;
                    if attr.key.0 == b"id" {
                        let val = str::from_utf8(&attr.value).expect("id is not a number");
                        way_id = val.parse().expect("val is not a number");
                    }
                }
                // parse road
                loop {
                    match xml.read_event_into(&mut buf) {
                        Ok(Event::Empty(ref nd)) if nd.name().0 == b"nd" => {
                            for attr in nd.attributes() {
                                let attr = attr?;
                                if attr.key.0 == b"ref" {
                                    let val = str::from_utf8(&attr.value).expect("ref is not a number");
                                    node_refs.push(val.parse().expect("val is not a number"));
                                }
                            }
                        }
                        Ok(Event::End(ref e2)) if e2.name().0 == b"way" => break, // end of this <way>
                        Ok(Event::Eof) => break,
                        Ok(_) => (),
                        Err(e) => panic!("XML parsing wen't wrong {}", e)
                    }
                    buf.clear();
                }
                ways.push(model::Way::new(way_id, node_refs));
            },
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => panic!("XML parsing wen't wrong {}", e)
        }
        buf.clear();
    }
    Ok(ways)
}

// returns all the nodes
pub fn parse_nodes<R: BufRead>(reader: R) -> Result<Vec<Node>, OsmError> {
    let mut xml = Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut nodes: HashMap<u64, Node> = HashMap::new();
    
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name().0 == b"node" => {
                // parse the xml node
                let mut id_opt: Option<u64> = None;
                let mut lat_opt: Option<f64> = None;
                let mut lon_opt: Option<f64> = None;

                for attr_res in e.attributes() {
                    let attr = attr_res.expect("attribute parse errro");
                    let key = attr.key;
                    let val = str::from_utf8(&attr.value).expect("invalid UTF-8 in attribute");
                    match key.0 {
                        b"id" => id_opt = Some(val.parse().expect("id not a number")),
                        b"lat" => lat_opt = Some(val.parse().expect("lat not a float")),
                        b"lon" => lon_opt = Some(val.parse().expect("lon not a float")),
                        _ => {}
                    }
                    if let (Some(id), Some(lat), Some(lon)) = (id_opt,lat_opt, lon_opt) {
                        nodes.insert(id, Node::new(id, lat, lon));
                    }
                }
            },
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => panic!("XML parsing wen't wrong {}", e)
        }
        buf.clear();
    }

    Ok(nodes.into_values().collect())
}

    
pub fn parse_ways_with_tags(tag_filters: &[&str], reader: impl BufRead) -> Result<Vec<Way>,OsmError> {
    let mut xml = Reader::from_reader(reader);
    xml.trim_text(true);
    let mut buf = Vec::new();
    let mut ways = Vec::new();
    
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref way)) if way.name().0 == b"way" => {
                let mut node_refs: Vec<u64> = Vec::new();
                let mut way_id: u64 = 0;
                for attr in way.attributes() {
                    let attr = attr?;
                    if attr.key.0 == b"id" {
                        let val = str::from_utf8(&attr.value).expect("id is not a number");
                        way_id = val.parse().expect("val is not a number");
                    }
                }
                // parse road
                loop {
                    match xml.read_event_into(&mut buf) {
                        Ok(Event::Empty(ref nd)) if nd.name().0 == b"nd" => {
                            for attr in nd.attributes() {
                                let attr = attr?;
                                if attr.key.0 == b"ref" {
                                    let val = str::from_utf8(&attr.value).expect("ref is not a number");
                                    node_refs.push(val.parse().expect("val is not a number"));
                                }
                            }
                        }
                        Ok(Event::Empty(ref tag)) if tag.name().0 == b"tag" => {
                            let mut key: Option<String> = None;
                            for attr in tag.attributes() {
                                let attr = attr?;
                                if attr.key.0 == b"k" {
                                    key = Some(str::from_utf8(&attr.value).expect("key is not a string").to_string());
                                }
                            }
                            if let Some(k) = key {
                                if tag_filters.contains(&k.as_str()) {
                                    ways.push(model::Way::new(way_id, node_refs.clone()));
                                    break;
                                }
                            }
                        }
                        Ok(Event::End(ref e2)) if e2.name().0 == b"way" => break, // end of this <way>
                        Ok(Event::Eof) => break,
                        Ok(_) => (),
                        Err(e) => panic!("XML parsing wen't wrong {}", e)
                    }
                    buf.clear();
                }
            },
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => panic!("XML parsing wen't wrong {}", e)
        }
        buf.clear();
    }
    Ok(ways)
}








#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;


    #[test]
    fn test_parse_nodes() {
        let xml = r#"
            <osm>
                <node id="1" lat="52.5" lon="13.4"/>
                <node id="2" lat="52.6" lon="13.5"/>
                <node id="3" lat="52.7" lon="13.6"/>
            </osm>
        "#;
        let reader = BufReader::new(xml.as_bytes());
        let nodes = parse_nodes(reader).unwrap();
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_parse_ways() {
        let xml = r#"
            <osm>
                <way>
                    <nd ref="1"/>
                    <nd ref="2"/>
                </way>
                <way>
                    <nd ref="3"/>
                </way>
            </osm>
        "#;
        let reader = BufReader::new(xml.as_bytes());
        let ways = parse_ways(reader).unwrap();
        assert_eq!(ways.len(), 2);
        assert_eq!(ways[0].node_refs, vec![1, 2]);
        assert_eq!(ways[1].node_refs, vec![3]);
    }
    #[test]
    fn test_filter_nodes_on_ways() {
        let nodes = vec![
            Node::new(1, 52.5, 13.4),
            Node::new(2, 52.6, 13.5),
            Node::new(3, 52.7, 13.6),
            Node::new(4, 52.8, 13.1),
        ];
        let ways = vec![
            Way::new(1,vec![1, 2]),
            Way::new(2,vec![3]),
        ];
        let filtered_nodes = filter_nodes_on_ways(nodes.clone(), &ways);
        assert_eq!(filtered_nodes.len(), 3);
    }

    #[test]
    fn test_parse_ways_with_tag() {
        let xml = r#"
            <osm>
                <way>
                    <nd ref="1"/>
                    <nd ref="2"/>
                    <tag k="highway" v="residential"/>
                </way>
                <way>
                    <nd ref="3"/>
                    <tag k="highway" v="motorway"/>
                </way>
            </osm>
        "#;
        let reader = BufReader::new(xml.as_bytes());
        let ways = parse_ways_with_tags(&["highway"], reader).unwrap();
        assert_eq!(ways.len(), 2);
        assert_eq!(ways[0].node_refs, vec![1, 2]);
        assert_eq!(ways[1].node_refs, vec![3]);
    }

    #[test]
    fn test_parse_ways_with_tags() {
        let xml = r#"
            <osm>
                <way>
                    <nd ref="1"/>
                    <nd ref="2"/>
                    <tag k="highway" v="residential"/>
                </way>
                <way>
                    <nd ref="3"/>
                    <tag k="highway" v="motorway"/>
                </way>
                <way>
                    <nd ref="4"/>
                    <tag k="name" v="Main St"/>
                </way>
                <way>
                    <nd ref="5"/>
                    <tag k="nice" v="motorway"/>
                </way>
            </osm>
        "#;
        let reader = BufReader::new(xml.as_bytes());
        let ways = parse_ways_with_tags(&["highway","nice"], reader).unwrap();
        println!("ways: {:?}", ways);
        assert_eq!(ways.len(), 3);
        assert_eq!(ways[0].node_refs, vec![1, 2]);
        assert_eq!(ways[1].node_refs, vec![3]);
        assert_eq!(ways[2].node_refs, vec![5]);
    }
}

