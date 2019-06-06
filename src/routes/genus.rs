use std::io::Error;
use rocket::{get, post};
use uuid::Uuid;
use crate::db::PrimaryDb;
use rusted_cypher::{GraphError};
use rusted_cypher::cypher::CypherResult;
use rusted_cypher::cypher::result::RowResult;


fn errorResult() -> Vec<RowResult> {
    let emptyResult = RowResult { row:  };
    return vec![emptyResult];
}

#[derive(Serialize, Deserialize)]
struct Genus {
    id: String,
    name: String,
}

#[post("/genus/create/<name>")]
pub fn create(conn: PrimaryDb, name: String) -> Result<String, Error> {
    let id: String = Uuid::new_v4().to_string();
    let statement = format!("CREATE (g:Genus {{ id: {:?}, name: {:?} }}) RETURN g AS genus", id, name);
    let data = match conn.exec(statement) {
        CypherResult(data) => data,
        GraphError => errorResult,
    };

    let mut genus: Genus;
    for row in data.rows() {
        genus = Genus {
            id: row.get("genus.id")?,
            name: row.get("genus.name")?
        }
    }

    return Ok("Done".to_string());
}
