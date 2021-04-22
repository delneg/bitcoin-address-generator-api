extern crate secp256k1;

use jelly::prelude::*;
use jelly::Result;
use std::fmt::Write;
use jelly::serde::{Deserialize, Serialize};
use jelly::actix_web::{HttpRequest, HttpResponse};
use bitcoin::secp256k1::Secp256k1;
use rand::thread_rng;

const ADDRESS_LENGTH: usize = 40;
const ADDRESS_BYTES: usize = ADDRESS_LENGTH / 2;
const KECCAK_OUTPUT_BYTES: usize = 32;
const ADDRESS_BYTE_INDEX: usize = KECCAK_OUTPUT_BYTES - ADDRESS_BYTES;

#[derive(Debug, Serialize, Deserialize)]
struct CryptoAddress {
    public_key: String,
    private_key: String,
}

fn to_hex_string(slice: &[u8], expected_string_size: usize) -> String {
    let mut result = String::with_capacity(expected_string_size);

    for &byte in slice {
        write!(&mut result, "{:02x}", byte).expect("Unable to format the public key.");
    }

    result
}

pub async fn generate_eth(request: HttpRequest) -> Result<HttpResponse> {
    let s = Secp256k1::new();
    let (private_key, public_key) = s.generate_keypair(&mut thread_rng());
    let public_key_array = &public_key.serialize_uncompressed()[1..];
    let keccak = tiny_keccak::keccak256(public_key_array);
    let address = to_hex_string(&keccak[ADDRESS_BYTE_INDEX..], 40);  // get rid of the constant 0x04 byte
    let eth_address = CryptoAddress { public_key: "0x".to_owned() + &address, private_key: to_hex_string(&private_key[..], 64) };
    request.json(200, eth_address)
}


pub async fn generate_btc(request: HttpRequest) -> Result<HttpResponse> {
    let s = Secp256k1::new();
    let keypair = s.generate_keypair(&mut thread_rng());
    let btc_address = CryptoAddress { public_key: keypair.1.to_string(), private_key: keypair.0.to_string() };

    request.json(200, btc_address)
}
