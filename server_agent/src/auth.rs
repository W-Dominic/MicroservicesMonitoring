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
    let mac_lower = &mac.to_lowercase();
    
    let mut hasher = Sha256::new();
    hasher.input(payload);
    let hash = hasher.result();
    let hash_str = format!("{:x}", hash); // convert to hex string
    
    println!("{:?}", &mac_lower);
    println!("{:?}", &hash_str);

    if hash_str[..] == mac_lower[..] {
        return heartbeat();
    }
    return Json("Not Authorized".to_string());
}

// ---Heartbeat function---
// get list of servers from server state
// determine if the endpoints on those servers are alive
// return output as Json Response 
fn heartbeat() -> Json<String> {
    return Json("TODO: Not yet Implemented".to_string());
}