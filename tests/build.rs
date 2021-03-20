use rocket::local::Client;
use rocket::http::{Header, Status};
use rocket::response::status;
use std::fs;

use build_tomo_rs;

#[test]
fn test_healthz() {

    let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");
    let response = client.get("/healthz").dispatch();
    assert_eq!(response.status(), Status::Ok);
}


// doesn't seem to be testable; secret always set
// #[test]
// fn test_build_no_secret() {
//     std::env::remove_var("GITHUB_WEBHOOK_SECRET");
//     let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");
//     let response = client.post("/build")
//         .header(Header::new("X-HUB-SIGNATURE-256", "b73839efb85fe05c79bee1cec3b29ecc0639074c97d27bd74aed202ed5c415fb"))
//         .dispatch();
//     assert_eq!(response.status(), Status::InternalServerError);
// }


#[test]
fn test_build_no_signature() {
    std::env::set_var("GITHUB_WEBHOOK_SECRET", "asdfASDF1234");
    let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");
    let response = client.post("/build")
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test] #[ignore]
fn test_build_success() {
    std::env::set_var("GITHUB_WEBHOOK_SECRET", "asdfASDF1234");
    let mut payload = fs::read_to_string("tests/data/github_webhook_payload.json").unwrap();
    let client = Client::new(build_tomo_rs::rocket()).expect("valid rocket instance");
    payload = fs::read_to_string("tests/data/github_webhook_payload.json").unwrap();
    let response = client.post("/build")
        .body(&payload)
        .header(Header::new("X-HUB-SIGNATURE-256", "b73839efb85fe05c79bee1cec3b29ecc0639074c97d27bd74aed202ed5c415fb"))
        .dispatch();

    assert_eq!(response.status(), Status::Accepted);
}
