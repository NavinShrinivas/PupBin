use std::collections::HashMap;
use hyper::{Response, Body};
use crate::entity::paste;
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;

pub async fn new_paste_handler( body_obj : HashMap<String,String>, db_conn : &DatabaseConnection ) -> Option<Response<Body>>{
    println!("{:?}",body_obj);
    let needed_fields = vec![ "paste_data" , "expiry_date"];
    for (_i,val) in needed_fields.into_iter().enumerate(){
        match body_obj.get_key_value(val){
            Some(_) => {
                continue;
            }
            _ => {
                return None;
            }
        }
    }
    let model = paste::Model{
        url_hash : String::from("A2X34"), //Need to get from key_gen service
        paste_content : match body_obj.get_key_value("post_data"){
            Some(content) => content.1.to_string(),
            _ => return None
        },
        deletion_date : DateTime::new(ChronoDate::from_ymd(2022,07,29),ChronoTime::from_hms(4,30,0))
    };
    let model : paste::ActiveModel = model.into();
    model.insert(db_conn).await.unwrap();

    Some(Response::new(Body::from("All needed fileds found")))
}
