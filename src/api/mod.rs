//! Main api

use jelly::actix_web::web::{resource, scope, ServiceConfig};

mod views;

pub fn configure(config: &mut ServiceConfig) {

    config.service(scope("/api")
        // Index
        .service(resource("/generate/btc").to(views::generate_btc))
        .service(resource("/generate/eth").to(views::generate_eth))
    );
}
