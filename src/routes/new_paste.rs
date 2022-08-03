use crate::entity::paste;
use crate::utils::utils_func;
use chrono::prelude::*;
use chrono::Duration;
use hyper::{Body, Response};
use sea_orm::entity::prelude::DateTime as SEAORMDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use RKGS_rust;

#[derive(serde::Serialize)]
struct NewPasteSuccessResponse {
    status: bool,
    key: String,
}

pub async fn new_paste_handler(
    body_obj: HashMap<String, String>,
    db_conn: &DatabaseConnection,
) -> Result<Response<Body>, String> {
    let needed_fields = vec!["paste_data", "lifetime"];
    for (_i, val) in needed_fields.into_iter().enumerate() {
        match body_obj.get_key_value(val) {
            Some(_) => {
                continue;
            }
            _ => {
                return Err(String::from("All needed fields not provided"));
            }
        }
    }
    let lifetime_i32 = match body_obj.get_key_value("lifetime") {
        Some(n) => n.1.trim(),
        None => return Err(String::from("All needed fields not provided")),
    };

    let mut lifetime_i32: i32 = match lifetime_i32.parse() {
        Ok(n) => n,
        Err(e) => {
            println!("Error : {}", e);
            return Err(String::from("Expriy date corrupted, parsing error"));
        }
    };
    let mut lifetime_minute = lifetime_i32 % 10;
    lifetime_i32 /= 10;
    lifetime_minute += (lifetime_i32 % 10) * 10;
    lifetime_i32 /= 10;
    let lifetime_hour = lifetime_i32;
    let expiry_datetime = Utc::now()
        + Duration::hours(lifetime_hour.into())
        + Duration::minutes(lifetime_minute.into());

    let key =
        match RKGS_rust::generate_key(String::from("0.0.0.0:5001"), String::from("5keybeta"), 5) {
            Ok(key) => {
                println!("New url generated : {}", key);
                key
            }
            Err(e) => {
                println!("{}", e);
                return Err(String::from(
                    "We are facing some trouble on our side, please try again in a while.",
                ));
            }
        };

    //Once we are using the KGS that only returns unique keys, we can remove this check, Can be used when
    //we come to custom url's
    /*
     *match Paste::find_by_id(String::from("A2X34")).one(db_conn).await {
     *    Ok(_) => {
     *        return Err(String::from("requested url already exists"));
     *    }
     *    Err(_) => {}
     *};
     */
    //==============================================================================
    let model = paste::Model {
        url_hash: key.clone(), //Need to get from key_gen service
        paste_content: match body_obj.get_key_value("paste_data") {
            Some(content) => content.1.to_string(),
            _ => return Err(String::from("All needed fields not provided")),
        },
        deletion_date: SEAORMDateTime::new(
            ChronoDate::from_ymd(
                expiry_datetime.year(),
                expiry_datetime.month(),
                expiry_datetime.day(),
            ),
            ChronoTime::from_hms(
                expiry_datetime.hour(),
                expiry_datetime.minute(),
                expiry_datetime.second(),
            ),
        ),
    };
    let model: paste::ActiveModel = model.into();
    //commented below for testing and building purposes only
    match model.insert(db_conn).await {
        Ok(_n) => {}
        Err(e) => {
            println!("Databse error : {}", e);
            return Err(String::from(
                "Uh oh! Try after some time, we are facing issues on our end",
            ));
        }
    };
    let response_obj = NewPasteSuccessResponse { status: true, key };
    Ok(utils_func::success_status_response(response_obj))
}
