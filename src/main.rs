#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use rocket_contrib::serve::{StaticFiles};
mod graphql;
use graphql::schema::{Query, Mutation, Schema};
mod db;
mod routes;

fn main() {
    rocket::ignite()
        .attach(db::PrimaryDb::fairing())
        .manage(Schema::new(
            Query,
            Mutation,
        ))
        .mount("/", routes![
            routes::get_graphql_handler,
            routes::post_graphql_handler,
            routes::graphiql
        ])
        .mount("/", StaticFiles::from("ui/public"))
        .launch();
}
