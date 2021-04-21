extern crate secp256k1;
use jelly::prelude::*;
use jelly::Result;
use jelly::serde::{Deserialize, Serialize};
use jelly::actix_web::{HttpRequest, HttpResponse};
use jelly::request::DatabasePool;
use jelly::tera::Context;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use bitcoin::secp256k1::Secp256k1;
use rand::thread_rng;

#[derive(Debug, Serialize, Deserialize)]
struct BtcAddress {
    public_key: String,
    private_key: String
}

pub async fn generate(request: HttpRequest) -> Result<HttpResponse> {

    let s = Secp256k1::new();
    let keypair = s.generate_keypair(&mut thread_rng());
    let btc_address  = BtcAddress { public_key: keypair.1.to_string(), private_key: keypair.0.to_string()};

    request.json(200, btc_address)
}
