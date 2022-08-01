use crate::entity::paste::Entity as Paste;
use crate::utils::utils_func;
use hyper::{Body, Response};
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use RKGS_rust;

#[derive(serde::Serialize)]
struct GetPasteSuccessResponse {
    status: bool,
    paste_content: String,
    paste_key: String,
}

pub async fn get_paste_handler(
    body_obj: HashMap<String, String>,
    db_conn: &DatabaseConnection,
) -> Result<Response<Body>, String> {
    let needed_fields = vec!["paste_key"];

    for (_i, value) in needed_fields.iter().enumerate() {
        match body_obj.get_key_value(&value.to_string()) {
            Some(_) => {
                continue;
            }
            None => return Err(String::from("Not all needeed fields are given")),
        };
    }

    match RKGS_rust::check_key(
        String::from("0.0.0.0:5001"),
        String::from("5keybeta"),
        body_obj.get("paste_key").unwrap().to_string(),
    ) {
        Ok(exists) => {
            if exists {
                match Paste::find_by_id(body_obj.get("paste_key").unwrap().to_string())
                    .one(db_conn)
                    .await
                {
                    Ok(paste_obj) => match paste_obj {
                        Some(paste_obj) => {
                            let ret_obj = GetPasteSuccessResponse {
                                status: true,
                                paste_content: paste_obj.paste_content,
                                paste_key: body_obj.get("paste_key").unwrap().to_string(),
                            };
                            return Ok(utils_func::success_status_response(ret_obj));
                        }
                        None => return Err(String::from("KEYNOTFOUND")),
                    },
                    Err(_) => return Err(String::from("INTERNALERROR/KEYNOTFOUND")),
                };
            } else {
                return Err(String::from("KEYNOTFOUND"));
            }
        }
        Err(e) => {
            println!("Ran into an error, from RKGS driver : {}", e);
            return Err(String::from(
                "Try again in a while, we are facing error on our side.",
            ));
        }
    }
}
