use http::{Request, Response, StatusCode, header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Value};


#[derive(Serialize, Deserialize)]
struct Crop {
    name: String,
    color: String,
}

fn handler(request: Request<()>) -> http::Result<Response<String>> {

    let crop = Crop {
        name: "Broccoli".to_owned(),
        color: "Usually Green".to_owned(),
    }

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/json")
        .body(serde_json::to_string(&crop)?)
        .expect("failed to render response");

    return Ok(response);
}
