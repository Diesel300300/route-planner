pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


use std::{
    collections::HashMap, 
    io::BufReader,
    fs::File,
    str,
};
use quick_xml::Reader;
use quick_xml::events::Event;


#[derive(Debug)]
struct Node {
    id: u64,
    lat: f64, 
    lon: f64
}


fn main() {
    let file = File::open("./data/map").expect("Couldn't open map file");
    
    let reader = BufReader::new(file);  
    
    let mut xml = Reader::from_reader(reader);
    xml.trim_text(true);
    
    let mut buf = Vec::new();

    let mut nodes: HashMap<u64, Node> = HashMap::new();
    nodes.reserve(100_000);

    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name().0 == b"node" => {
                let mut id_opt: Option<u64> = None;
                let mut lat_opt: Option<f64> = None;
                let mut lon_opt: Option<f64> = None;

                for attr_res in e.attributes() {
                    let attr = attr_res.expect("attribute parse error");
                    let key = attr.key;
                    let val = str::from_utf8(&attr.value).expect("invalid UTF-8 in attribute");

                    match key.0 {
                        b"id" => id_opt = Some(val.parse().expect("id not a number")),
                        b"lat" => lat_opt = Some(val.parse().expect("lat not a float")),
                        b"lon" => lon_opt = Some(val.parse().expect("lon not a float")),
                        _ => {}
                    }
                    if let (Some(id), Some(lat), Some(lon)) = (id_opt, lat_opt, lon_opt) {
                        nodes.insert(id, Node {id, lat, lon});
                    }
                };
            },
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => panic!("XML error: {}", e)
        };
        buf.clear();
    }
    if let Some((id, node)) = nodes.iter().next() {
        println!("Sample node - id: {}, lat: {}, lon: {}",
            id, node.lat, node.lon
        )
    }
}
