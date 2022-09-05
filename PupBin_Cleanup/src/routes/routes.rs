use crate::utils::{text_assets, utils_func};
use hyper::Method;
use hyper::{body::HttpBody, Body, Request, Response};
use sea_orm::DatabaseConnection;
use serde_urlencoded::de;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

pub async fn router_function(
    req: Request<Body>,
    db_conn: Arc<DatabaseConnection>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, "/") => {
            //Preflight for CORS check
            //For clean up service we are restricting the origin domains
            //only from localhost
            return Ok(utils_func::preflight_response());
        }
        (&Method::GET, "/") => Ok(Response::new(Body::from(text_assets::home_text_asset()))),
        (&Method::DELETE, "/") => {
            println!("Request to delete recieved, processing...");
            //Auth check :
            let body = match req.into_body().data().await {
                Some(Ok(data)) => data.to_vec(),
                _ => {
                    return Ok(utils_func::failed_status_response(String::from(
                        "Failed authentication",
                    )));
                }
            };
            //Access string is stored in a file in FS with perm restrictions.
            //Can only be read and written by owner

            let body_obj: HashMap<String, String> = match de::from_bytes(&body) {
                Ok(map) => map,
                _ => {
                    return Ok(utils_func::failed_status_response(String::from(
                        "Error parsing body!",
                    )));
                }
            };

            Ok(utils_func::success_status_response(String::from(
                "Auth Success",
            )))
        }
        _ => {
            let response = utils_func::failed_status_response(String::from("INVALIDPATH"));
            Ok(response)
        }
    }
}
