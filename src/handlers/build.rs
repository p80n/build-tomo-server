use log::{ debug, info, error };

use rocket::{Data, Outcome};
use rocket::data;
use rocket::data::FromDataSimple;
use rocket::http::Status;
use rocket::request::Request;
//use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
//use tinytemplate::TinyTemplate;
//use std::error::Error;
use rocket::response::Debug;


#[derive(Debug, Serialize)]
struct Context {
    clone_url: String,
    build_id: String,
    build_image: String,
    command: String,
    build_mount_path: String
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    clone_url: String
}

#[derive(Debug, Deserialize)]
struct HeadCommit {
    id: String
}

#[derive(Debug, Deserialize)]
struct GitHubEvent {
    repository: Repository,
    head_commit: HeadCommit
}



/// Payload dataguard; verifies GitHub signature
#[derive(Debug, PartialEq)]
pub struct SignedPayload(pub String);

impl FromDataSimple for SignedPayload {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<SignedPayload, ()> {
        let secret = match std::env::var("GITHUB_WEBHOOK_SECRET") {
            Ok(s) => s,
            Err(_) => {
                error!("Unable to verify payload. Please set an environment value for GITHUB_WEBHOOK_SECRET");
                return Outcome::Failure((Status::InternalServerError, ())); }
        };
        let signature = match request.headers().get_one("X-HUB-SIGNATURE-256") {
            Some(header) => header,
            None => { error!("Missing X-HUB-SIGNATURE-256 header");
                      return Outcome::Failure( (Status::Unauthorized, ()) ); }
        };

        let mut body = String::new();
        if let Err(e) = data.open().read_to_string(&mut body) {
            error!("Unable to read request payload: {:?}", e);
            return Outcome::Failure((Status::InternalServerError, ()));
        }

        if !is_valid_signature(&signature, &body, &secret) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        Outcome::Success(SignedPayload(body))
    }
}

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
fn is_valid_signature(signature: &str, body: &str, secret: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();
    mac.update(body.as_bytes());
    match hex::decode(signature) {
        Ok(decoded) => {
            match mac.verify(&decoded) {
                Ok(_) => true,
                Err(e) => {
                    error!("Error verifying payload signature {:?}: {:?}", signature, e);
                    false
                }
            }
        },
        Err(e) => {
            error!("Error converting payload signature to hex: {:?}", e);
            false
        }
    }
}

use futures::{StreamExt, TryStreamExt};
use rocket::response::status;
use futures::executor::block_on;
// Result<(), Debug<kube::Error>>



#[post("/build", data = "<payload>")]
pub fn build(payload: SignedPayload) -> Status
{

    let event: GitHubEvent = serde_json::from_str(payload.0.as_str()).unwrap();
    info!("Received GitHub event for {:?}", event.repository.full_name);
    info!("{:?}", event);




    match deploy_build_job(&event) {
        Ok(_) => Status::Accepted,
        Err(e) => {
            error!("{:?}", e);
            Status::InternalServerError
        }
    }
}




use kube::api::{Api, ListParams, PostParams, Meta, WatchEvent };
use kube::{Config, Client};
use k8s_openapi::api::batch::v1::Job;


#[tokio::main]
async fn deploy_build_job(event: &GitHubEvent) -> Result<(), kube::Error>
{
    let client = Client::try_default().await?;
    let config = Config::infer().await?;

    let job_id = &event.head_commit.id[0..9];
    let job_name = format!("build-job-{}", job_id);
    // wget https://raw.githubusercontent.com/username/reponame/path/to/.build-tomo

    let my_job = serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": job_name,
        },
        "spec": {
            "template": {
                "metadata": {
                    "name": job_name
                },
                "spec": {
                    "containers": [{
                        "name": "empty",
                        "image": "alpine/git:latest",
                        "command": ["sleep", "15"]
                    }],
                    "restartPolicy": "Never",
                }
            }
        }
    }))?;

    let jobs: Api<Job> = Api::namespaced(client, &config.default_ns);
    let pp = PostParams::default();
    jobs.create(&pp, &my_job).await?;

    Ok(())
}

