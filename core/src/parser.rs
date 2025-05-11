use core::str;
use std::collections::HashSet;
use std::panic;
use std::{
    collections::HashMap, 
    io::BufRead,
};
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::model::{ self, Node, Way };



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
pub fn parse_ways<R: BufRead>(reader: R) -> anyhow::Result<Vec<Way>> {
    let mut xml = Reader::from_reader(reader);
    xml.trim_text(true);
    let mut buf = Vec::new();
    let mut ways = Vec::new();
    
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().0 == b"way" => {
                let mut node_refs: Vec<u64> = Vec::new();
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
                ways.push(model::Way::new(node_refs));
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
pub fn parse_nodes<R: BufRead>(reader: R) -> anyhow::Result<Vec<Node>> {
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


