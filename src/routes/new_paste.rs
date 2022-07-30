use std::collections::HashMap;
use hyper::{Response, Body};
use crate::entity::paste;
use sea_orm::entity::prelude::*;
use sea_orm::entity::prelude::DateTime as SEAORMDateTime;
use sea_orm::DatabaseConnection;
use chrono::prelude::*;
use chrono::Duration;
use crate::entity::paste::Entity as Paste;
use std::net::UdpSocket;

pub async fn new_paste_handler( body_obj : HashMap<String,String>, db_conn : &DatabaseConnection ) -> Result<Response<Body>, String>{
    let needed_fields = vec![ "paste_data" , "lifetime"];
    for (_i,val) in needed_fields.into_iter().enumerate(){
        match body_obj.get_key_value(val){
            Some(_) => {
                continue;
            }
            _ => {
                return Err(String::from("All needed fields not provided"));
            }
        }
    }
    let lifetime_i32 = match body_obj.get_key_value("lifetime"){
        Some(n) => { n.1.trim() },
        None => { return Err(String::from("All needed fields not provided")) }
    };

    let mut lifetime_i32 : i32 = match lifetime_i32.parse(){
        Ok(n) => { n },
        Err(e) =>{
            println!("Error : {}",e);
            return Err(String::from("Expriy date corrupted, parsing error"))
        }   
    };
    let mut lifetime_minute =  lifetime_i32%10; 
    lifetime_i32 /= 10;
    lifetime_minute +=  (lifetime_i32%10)*10; 
    lifetime_i32 /= 10;
    let lifetime_hour = lifetime_i32;
    let expiry_datetime = Utc::now() + Duration::hours(lifetime_hour.into()) + Duration::minutes(lifetime_minute.into());



    //Once we are use the KGS only returns unique keys, we can remove this check, Can be used when
    //we come to custom url's
    /*
     *match Paste::find_by_id(String::from("A2X34")).one(db_conn).await{
     *    Ok( _ ) => { 
     *        return Err(String::from("requested url already exists"));
     *    },
     *    Err(_) => { }
     *};
     */
    //==============================================================================

    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    socket.connect("0.0.0.0:5001").expect("Connection to KGS failed, Maybe KGS is not turned on");
    socket.send(String::from("{\"Work\":\"generate\", \"Pool\":\"5key\", \"Url\":\"\"}").as_bytes()).expect("Error sending messsage to KGS");

    let model = paste::Model{
        url_hash : String::from("A2X34"), //Need to get from key_gen service
        paste_content : match body_obj.get_key_value("paste_data"){
            Some(content) => content.1.to_string(),
            _ => return Err(String::from("All needed fields not provided"))
        },
        deletion_date : SEAORMDateTime::new(ChronoDate::from_ymd(expiry_datetime.year(),expiry_datetime.month(),expiry_datetime.day()),
                                            ChronoTime::from_hms(expiry_datetime.hour(),expiry_datetime.minute(),expiry_datetime.second()))
    };
    let model : paste::ActiveModel = model.into();
    //commented below for testing and building purposes only
    /*
     *match model.insert(db_conn).await{
     *    Ok(_n) => {  },
     *    Err(e) => { 
     *        println!("Databse error : {}",e);
     *        return Err(String::from("Uh oh! Try after some time, we are facing issues on our end"));
     *    }
     *};
     */

    Ok(Response::new(Body::from("Paste created successfully"))) //Should also return link for paste.
}
