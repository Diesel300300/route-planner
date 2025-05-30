#[macro_use] extern crate rocket;

use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_cors::{CorsOptions, AllowedOrigins};
use rocket::serde::json::Json;
use rocket::State;

use serde::Deserialize;

use route_parser::graph::Graph;
use route_parser::model::{Way,Path};
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


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PathsRequest {
    start_lat: f64,
    start_lon: f64,
    goal_lat: f64,
    goal_lon: f64,

    amount: u16,
    target_distance: f64,
}


#[post("/ways_by_tags", format = "json", data = "<req>")]
async fn ways_by_tags(req: Json<TagsRequest>) -> Result<Json<Vec<Way>>,Custom<String>> {
    let file_path = "data/map";

    let tag_slices: Vec<&str> = req.tags.iter().map(String::as_str).collect();
    
    let ways = parse_osm_ways(file_path, &tag_slices)
        .map_err(|e| Custom(Status::InternalServerError, format!("Error parsing ways: {}", e)))?;
    Ok(Json(ways))
}


#[post("/paths_bfs", format = "json", data = "<req>")]
async fn paths(graph: &State<Graph>, req: Json<PathsRequest>) -> Result<Json<Vec<Path>>,Custom<String>> {
    // defaul tol = 200 meters
    // don't want the user to decide the tolerance
    let paths = graph.get_paths_bfs(req.start_lat, req.start_lon, req.goal_lat, req.goal_lon, req.amount as usize, req.target_distance, 200.0);
    Ok(Json(paths))
}

#[post("/paths_special_dijkstra", format = "json", data = "<req>")]
async fn paths_special_dijkstra(graph: &State<Graph>, req: Json<PathsRequest>) -> Result<Json<Vec<Path>>,Custom<String>> {
    // defaul tol = 200 meters
    // don't want the user to decide the tolerance
    let paths = graph.get_paths_special_dijkstra(req.start_lat, req.start_lon, req.goal_lat, req.goal_lon, req.amount as usize, req.target_distance, 200.0);
    Ok(Json(paths))
}

#[post("/paths_dfs", format = "json", data = "<req>")]
async fn paths_dfs(graph: &State<Graph>, req: Json<PathsRequest>) -> Result<Json<Vec<Path>>,Custom<String>> {
    // defaul tol = 200 meters
    // don't want the user to decide the tolerance
    let paths = graph.get_paths_dfs(req.start_lat, req.start_lon, req.goal_lat, req.goal_lon, req.amount as usize, req.target_distance, 200.0);
    if let Some(paths) = paths {
        Ok(Json(paths))
    } else {
        Err(Custom(Status::NotFound, "No paths found".to_string()))
    }
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
        .mount("/", routes![ways_by_tags, paths, paths_special_dijkstra, paths_dfs])
}
