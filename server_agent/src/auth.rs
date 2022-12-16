use actix_web::{
    get,
    // post,
    // put,
    // error::ResponseError,
    // web::Path,
    web::Json,
    // web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::str;
// use hex_literal::hex;

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    payload: String,
    mac: String,
}

#[get("/auth")]
pub async fn get_auth(body: Json<AuthRequest>) -> Json<String> {
    let payload = &body.payload;
    let mac = &body.mac;
    
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let result = hasher.finalize();
    
    /*
    let result_str= match str::from_utf8(&result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid Utf-8 sequence: {}", e),
    };
    */

    println!("{:?}", &mac);
    // println!("{:?}", &mac.as_bytes()[..]);
    // println!("{:?}", &result_str);

    if result[..] == mac.as_bytes()[..] {
        return Json("Authorized".to_string());
    }
    return Json("Not Authorized".to_string());
}