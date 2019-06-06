#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
use rocket::routes;
use rocket_contrib::serve::{StaticFiles};
mod db;
mod routes;

fn main() {
    rocket::ignite()
        .attach(db::PrimaryDb::fairing())
        .mount("/api", routes![
            routes::genus::create,
        ])
        .mount("/", StaticFiles::from("ui/public"))
        .launch();
}
