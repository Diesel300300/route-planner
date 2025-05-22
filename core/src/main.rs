#[macro_use] extern crate rocket;

use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_cors::{CorsOptions, AllowedOrigins};

use rocket::serde::json::Json;
use serde::Deserialize;
use route_parser::model::Way;
use route_parser::{parse_osm_ways, create_graph};

pub const ACCEPTED_ROAD_TYPES: &[&str] = &[
    "residential",
    "unclassified",
    "track",
    "service",
    "tertiary",
    "road",
    "secondary",
    "primary",
    "trunk",
    "primary_link",
    "trunk_link",
    "tertiary_link",
    "secondary_link",
    "highway",
];


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TagsRequest {
    tags: Vec<String>,
}


#[post("/ways_by_tags", format = "json", data = "<req>")]
async fn ways_by_tags(req: Json<TagsRequest>) -> Result<Json<Vec<Way>>,Custom<String>> {
    let file_path = "data/map";

    let tag_slices: Vec<&str> = req.tags.iter().map(String::as_str).collect();
    
    let ways = parse_osm_ways(file_path, &tag_slices)
        .map_err(|e| Custom(Status::InternalServerError, format!("Error parsing ways: {}", e)))?;
    Ok(Json(ways))
}



#[launch]
fn rocket() -> _ {
    let graph = create_graph("data/map", ACCEPTED_ROAD_TYPES).expect("Failed to create graph");

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS options");

    rocket::build()
        .manage(graph)
        .attach(cors)
        .mount("/", routes![ways_by_tags])
}
