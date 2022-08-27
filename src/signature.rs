use base64::encode;
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::utils::get_ts;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug)]
pub struct Signature {
    pub ts: u128,
    pub msg: String,
}

pub fn sign_url(url: &str) -> Signature {
    let key = "aVwDprJBYvnz1NBs8W7GBuaHQDeoynolGF5IdsxyYP6lyCzxAOG38hleJo43NnB6";

    let ts = get_ts();

    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("hmac error");
    mac.update(&url.chars().take(255).collect::<String>().as_bytes());
    mac.update(ts.to_string().as_bytes());

    let result = mac.finalize().into_bytes();

    Signature {
        ts,
        msg: encode(&result[..]),
    }
}
