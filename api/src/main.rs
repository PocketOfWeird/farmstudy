#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::databases::rusted_cypher;

#[database("neo4j")]
pub struct GraphDBConn(rusted_cypher::GraphClient);


#[get("/")]
fn index(conn: GraphDBConn) -> &'static str {
    let result = conn.exec("MATCH (n:Crop) RETURN n").unwrap();
    println!("{:?}", result);
    return "Done";
}

fn main() {
    rocket::ignite()
        .attach(GraphDBConn::fairing())
        .mount("/", routes![index])
        .launch();
}
