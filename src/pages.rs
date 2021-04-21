use jelly::actix_web::web::{resource, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;
use jelly::actix_web::{HttpRequest, HttpResponse, web::Form};
use jelly::tera::Context;
pub async fn homepage(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "index.html", {
        let context = Context::new();
        context
    })
}

pub fn configure(config: &mut ServiceConfig) {    
    config.service(resource("/").to(homepage));
}
