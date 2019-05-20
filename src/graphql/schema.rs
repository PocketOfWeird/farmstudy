use juniper::{Context as JuniperContext, FieldResult, RootNode};
use uuid::Uuid;
use crate::db;
use crate::graphql::models;

pub struct Context {
    pub connection: db::PrimaryDb
}

impl JuniperContext for Context {}

pub struct Query;

#[juniper::GraphQLObject(Context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        return "1.0";
    }

    fn genus(context: &Context, id: String) -> FieldResult<models::Genus> {
        let statement = format!("MATCH (n:Genus {{ id: {:?} }}) RETURN n", id.cloned());
        let genus = context.connection.exec(statement).unwrap();
        return Ok(genus);
    }
}

pub struct Mutation;

#[juniper::GraphQLObject(Context = Context)]
impl Mutation {
    fn newGenus(context: &Context, input_genus: models::InputGenus) -> FieldResult<models::Genus> {
        let statement = format!("CREATE (g:Genus {{ id: {:?}, name: {:?} }})", Uuid::new_v4().to_string(), input_genus.name.cloned());
        let genus = context.connection.exec(statement).unwrap();
        return Ok(genus);
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
