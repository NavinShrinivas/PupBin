use super::{get_paste, new_paste};
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
            return Ok(utils_func::preflight_response());
        }
        (&Method::GET, "/") => Ok(Response::new(Body::from(text_assets::home_text_asset()))),
        (&Method::POST, "/newPaste") | (&Method::POST, "/newPaste/") => {
            let body = match req.into_body().data().await {
                Some(Ok(data)) => data.to_vec(),
                _ => {
                    return Ok(utils_func::failed_status_response(String::from(
                        "Please provide a body",
                    )))
                }
            };
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
        (&Method::GET, "/getPaste") | (&Method::GET, "/getPaste/") => {
            let req_clone = req.uri().clone();
            let body = match req.into_body().data().await {
                Some(Ok(data)) => {
                    let b = data.to_vec();
                    match de::from_bytes::<HashMap<String, String>>(&b) {
                        Ok(something) => something,
                        Err(_) => {
                            return Ok(utils_func::failed_status_response(
                                "Error deserilazing body".to_string(),
                            ));
                        }
                    }
                }
                _ => {
                    match req_clone.query().map(|v| {
                        url::form_urlencoded::parse(v.as_bytes())
                            .into_owned()
                            .collect::<Vec<(String, String)>>()
                    }) {
                        Some(vec) => vec.into_iter().collect::<HashMap<String, String>>(),
                        None => {
                            return Ok(utils_func::failed_status_response(String::from(
                                "Please provide a body or get paramters",
                            )));
                        }
                    }
                }
            };

            match get_paste::get_paste_handler(body, db_conn.as_ref()).await {
                Ok(res) => Ok(res),
                Err(err_status) => {
                    return Ok(utils_func::failed_status_response(err_status));
                }
            }
        }
        (&Method::GET, "/install_script") | (&Method::GET, "/install_script/") => {
            Ok(utils_func::install_script())
        }
        (&Method::GET, "/install_script_test") | (&Method::GET, "/install_script_test/") => {
            Ok(utils_func::install_script_test())
        }
        _ => {
            let response = utils_func::failed_status_response(String::from("INVALIDPATH"));
            Ok(response)
        }
    }
}
