use std::collections::HashMap;
use hyper::{Response, Body};

pub fn new_paste_handler( mut body_obj : HashMap<String,String> ) -> Option<Response<Body>>{
    println!("{:?}",body_obj);
    return None;
}
