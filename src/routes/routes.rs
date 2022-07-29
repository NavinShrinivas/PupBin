use hyper::{Request, Body, Response, StatusCode,body::HttpBody};
use std::convert::Infallible;
use hyper::Method;
use crate::utils::{text_assets,utils_func};
use serde_urlencoded::de;
use std::collections::HashMap;
use super::new_paste;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub async fn router_function(req : Request<Body>, db_conn : Arc<DatabaseConnection>) -> Result<Response<Body>, Infallible>
{
    match (req.method(), req.uri().path()){
        (&Method::GET,"/") => {
            Ok(Response::new(Body::from(text_assets::home_text_asset())))
        },
        (&Method::POST, "/newPaste") => {
            let body = req.into_body().data().await.unwrap().unwrap().to_vec();
            let body_obj  = de::from_bytes::<HashMap<String,String>>(&body); 
            let body_obj = match body_obj{
                Ok(hash_map) => {
                    println!("New Paste!");
                    hash_map
                },
                Err(e) => {
                    eprintln!("Something went wrong deserilazing body : {}",e);
                    return Ok(utils_func::failed_status_response("Error deserilazing body".to_string()));
                }
            };
            match new_paste::new_paste_handler(body_obj,db_conn.as_ref()).await{
                Some(res) => {
                    Ok(res)
                },
                None => {
                    return Ok(utils_func::failed_status_response("Needed fields not found".to_string()));
                }
            }
            
        },
        _ => { 
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type" ," application/json")
                .body(Body::from(" { \"status\" : \"false\" , \"error\" : \"Invalid Path\" }")).unwrap();
            Ok(response)
        }
    }
}
