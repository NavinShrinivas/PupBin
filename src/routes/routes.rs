use super::new_paste;
use crate::utils::{text_assets, utils_func};
use hyper::Method;
use hyper::{body::HttpBody, Body, Request, Response, StatusCode};
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
        (&Method::GET, "/") => Ok(Response::new(Body::from(text_assets::home_text_asset()))),
        (&Method::POST, "/newPaste") => {
            let body = req.into_body().data().await.unwrap().unwrap().to_vec();
            let body_obj = de::from_bytes::<HashMap<String, String>>(&body);
            let body_obj = match body_obj {
                Ok(hash_map) => {
                    println!("New Paste!");
                    hash_map
                }
                Err(e) => {
                    eprintln!("Something went wrong deserilazing body : {}", e);
                    return Ok(utils_func::failed_status_response(
                        "Error deserilazing body".to_string(),
                    ));
                }
            };
            match new_paste::new_paste_handler(body_obj, db_conn.as_ref()).await {
                Ok(res) => Ok(res),
                Err(err_status) => {
                    return Ok(utils_func::failed_status_response(err_status));
                }
            }
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", " application/json")
                .body(Body::from(
                    " { \"status\" : \"false\" , \"error\" : \"Invalid Path\" }",
                ))
                .unwrap();
            Ok(response)
        }
    }
}
