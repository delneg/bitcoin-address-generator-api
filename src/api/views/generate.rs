use jelly::prelude::*;
use jelly::Result;
use jelly::serde::{Deserialize, Serialize};
use jelly::actix_web::{HttpRequest, HttpResponse};
use jelly::request::DatabasePool;
use jelly::tera::Context;

#[derive(Debug, Serialize, Deserialize)]
struct BtcAddress {
    public_key: String,
    private_key: String
}

/// Returns an overview of everything in the system.
pub async fn generate(request: HttpRequest) -> Result<HttpResponse> {

    let payload = BtcAddress{ public_key: String::from("pub"), private_key: String::from("priv")};
    request.json(200, payload)
    // request.render(200, "dashboard/index.html", {
    //     let context = Context::new();
    //     context
    // })
}
