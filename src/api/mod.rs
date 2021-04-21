//! Main api

use jelly::actix_web::web::{resource, scope, ServiceConfig};
use jelly::guards::Auth;

mod views;

pub fn configure(config: &mut ServiceConfig) {

    config.service(scope("/api")
        // Index
        .service(resource("/generate").to(views::generate))
    );
}
