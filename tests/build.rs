//use super::rocket;
use rocket::local::Client;
use rocket::http::{Header, Status};

use std::fs;
//use std::io;

use build_tomo_rs;

#[test]
fn test_healthz() {

    let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");
    let response = client.get("/healthz").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_build() {

    let mut payload = fs::read_to_string("tests/data/github_webhook_payload.json").unwrap();


    let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");

    let response = client.post("/build")
        .body(payload)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);

    // TODO figure out how to pass buy reference to /post?
    payload = fs::read_to_string("tests/data/github_webhook_payload.json").unwrap();
    let response = client.post("/build")
        .body(payload)
        .header(Header::new("X-HUB-SIGNATURE", "61ce7359acda0f8982b10dbe5f81920723fc31f7"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

}
