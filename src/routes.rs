use rocket::{get, post};
use rocket::response::content::Html;
use rocket::State;
use juniper_rocket::{GraphQLResponse, GraphQLRequest};
use crate::db::PrimaryDb;
use crate::graphql::schema::{Context, Schema};


#[get("/graphql?<request>")]
pub fn get_graphql_handler(context: PrimaryDb, request: GraphQLRequest, schema: State<Schema>) -> GraphQLResponse {
    return request.execute(&schema, &Context { connection: context });
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(context: PrimaryDb, request: GraphQLRequest, schema: State<Schema>) -> GraphQLResponse {
    return request.execute(&schema, &Context { connection: context });
}

#[get("/graphiql")]
pub fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}
