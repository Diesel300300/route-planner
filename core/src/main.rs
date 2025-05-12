#[macro_use] extern crate rocket;

use std::{fs::File, io::BufReader};
use rocket::response::status::Custom;
use rocket::http::Status;

use rocket::serde::json::Json;
use serde::Deserialize;
use route_parser::{model::Way, parser::{parse_ways_with_tags}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TagsRequest {
    tags: Vec<String>,
}


#[post("/ways_by_tags", format = "json", data = "<req>")]
async fn ways_by_tags(req: Json<TagsRequest>) -> Result<Json<Vec<Way>>,Custom<String>> {
    let file_path = "data/map";
    let file = File::open(file_path).expect(String::as_str(&format!("failed to open {}", file_path)));
    let reader = BufReader::new(file);

    let tag_slices: Vec<&str> = req.tags.iter().map(String::as_str).collect();
    
    let ways = parse_ways_with_tags(&tag_slices, reader)
        .map_err(|e| Custom(Status::InternalServerError, format!("Error parsing ways: {}", e)))?;
    Ok(Json(ways))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ways_by_tags])
}
