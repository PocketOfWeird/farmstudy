#[derive(juniper::GraphQLObject)]
#[graphql(description="A crop")]
pub struct Crop {
    pub id: String,
    pub farmId: String,
    pub name: String,
    pub genus: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description="A crop")]
pub struct InputCrop {
    pub name: String,
    pub genus: String,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A plant family/species name")]
pub struct Genus {
    pub id: String,
    pub farmId: String,
    pub name: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description="A plant family/species name")]
pub struct InputGenus {
    pub name: String,
}
