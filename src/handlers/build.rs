//use crate::config::AppState;
//use crate::errors::{Errors, FieldValidator};

//use rocket::State;
//use rocket_contrib::json::{Json, JsonValue};
//use serde::Deserialize;
//use validator::Validate;


use rocket::Data;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

#[derive(Debug)]
pub enum SignatureError {
    Missing,
//    Invalid
}

#[derive(Debug)]
pub struct XHubSignature(String);


impl<'a, 'r> FromRequest<'a, 'r> for XHubSignature {
    type Error = SignatureError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let signature = request.headers().get_one("X-HUB-SIGNATURE");
        match signature {
            Some(signature) => Outcome::Success(XHubSignature(signature.to_string())),
            None => Outcome::Failure( (Status::Unauthorized, SignatureError::Missing) ),
        }
    }
}

#[post("/build", data = "<payload>")]
pub fn build(payload: Data, signature: XHubSignature)
{

println!("{:?}", signature);




}


fn verify_github_signature(){


}
