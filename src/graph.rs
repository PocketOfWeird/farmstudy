use juniper::{FieldResult};

#[derive(juniper::GraphQLObject)]
#[graphql(description="A crop")]
struct Crop {
    id: String,
    farmId: String,
    name: String,
    genus: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description="A crop")]
struct InputCrop {
    name: String,
    genus: String,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A plant family/species name")]
struct Genus {
    id: String,
    farmId: String,
    name: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description="A plant family/species name")]
struct Genus {
    name: String,
}

struct Context {
    // Use your real database pool here.
    pool: DatabasePool,
}
