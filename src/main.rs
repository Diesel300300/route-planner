use std::fs::File;
use std::io::{BufReader};
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let file = File::open("./data/map").expect("Couldn't open map file");
    
    let reader = BufReader::new(file);  
    
    let mut xml = Reader::from_reader(reader);
    xml.trim_text(true);
    
    let mut buf = Vec::new();
    let mut count = 0_usize;

    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name().0 == b"node" => {
                count += 1;
            },
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => panic!("XML error: {}", e)
        };
        buf.clear();
    }

    println!("There are {} amount of nodes in the file", count);
}
