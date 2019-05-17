#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use uuid::Uuid;
use rocket::http::Status;
use rocket_contrib::databases::rusted_cypher::{GraphClient, GraphError};
use rocket_contrib::databases::rusted_cypher::cypher::CypherResult;
use rocket_contrib::json::{Json, JsonValue};

// Database Connection
#[database("neo4j")]
pub struct GraphDBConn(GraphClient);

// Data Models
#[derive(Serialize, Deserialize)]
struct Crop {
    id: Option<Uuid>,
    name: String,
    genus: Uuid,
}

#[derive(Serialize, Deserialize)]
struct Genus {
    id: Option<Uuid>,
    name: String
}

// Routes
#[get("/")]
fn index(conn: GraphDBConn) -> &'static str {
    let result = conn.exec("MATCH (n:Crop) RETURN n").unwrap();
    println!("{:?}", result);
    return "Done";
}

#[post("/genus/create", format = "json", data = "<data>")]
fn genus_create(conn: GraphDBConn, data: Json<Genus>) -> Result<CypherResult, GraphError> {
    let genus = Genus { id: Some(Uuid::new_v4()), name: data.name };
    let result = conn.exec(format!("CREATE (g:Genus {{ id: {:?}, name: {:?} }})", genus.id, genus.name))?;

    return result;
}


fn main() {
    rocket::ignite()
        .attach(GraphDBConn::fairing())
        .mount("/", routes![index])
        .launch();
}
