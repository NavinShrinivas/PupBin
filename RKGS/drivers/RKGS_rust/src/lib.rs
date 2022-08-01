use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Job{
    #[serde(alias = "Work")]
    work :  String,
    #[serde(alias = "Pool")]
    pool : String,
    #[serde(alias = "Url")]
    url : String,
    #[serde(alias = "Len")]
    len : String,
    #[serde(alias = "Error")]
    error : String
}

pub fn generate_key(kgs_addr : String, pool : String, len : i32) -> std::result::Result<String,String>{
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    socket.connect(&kgs_addr).expect("Connection to KGS failed, Maybe KGS is not turned on");
    let request_json_string = format!("{{\"Work\":\"generate\", \"Pool\":\"{}\", \"Len\":\"{}\", \"Url\":\"\", \"Error\":\"\"}}", pool, len);
    socket.send(request_json_string.as_bytes()).expect("Error sending messsage to kgs"); 
    let mut kgs_buf = vec![0;1024];
    socket.set_read_timeout(Some(std::time::Duration::from_millis(50))).expect("Internal code error"); //making sure we are bot held up
    let len = match socket.recv(&mut kgs_buf){
        Ok(n) => {
            n
        },
        Err(e) => { 
            println!("Error from key gen service : {}",e);
            return Err(String::from("We are facing some trouble on our side, please try again in a while."));
        }
    };

    let kgs_reply_obj : Job = match String::from_utf8(kgs_buf[0..len].to_vec()){
        Ok(kgs_reply) => { serde_json::from_str(&kgs_reply).expect("Error generated from serde in RKGS lib") },
        Err(e) => {
            println!("Error from kgs (returning invalif utf-8 chars) : {}",e);
            return Err(String::from("We are facing some trouble on our side, please try again in a while."));
        }
    };
    match kgs_reply_obj.work.as_str(){
        "generate" => { return Ok(kgs_reply_obj.url) },
        _ => { 
            let ret_err_string = match String::from_utf8(kgs_buf[0..len].to_vec()){
                Ok(original_err) => { original_err }
                Err(_) => {
                    String::from("Error from KGS : Please try again in a while, we are facing issues on our side")
                }
            };
            return Err(ret_err_string)
        }
    }
}


pub fn check_key(kgs_addr : String, pool : String, key : String) -> std::result::Result<bool,String>{
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    socket.connect(&kgs_addr).expect("Connection to KGS failed, Maybe KGS is not turned on");
    let request_json_string = format!("{{\"Work\":\"check\", \"Pool\":\"{}\", \"Len\":\"\", \"Url\":\"{}\", \"Error\":\"\"}}", pool, key);
    socket.send(request_json_string.as_bytes()).expect("Error sending messsage to kgs"); 
    let mut kgs_buf = vec![0;1024];
    socket.set_read_timeout(Some(std::time::Duration::from_millis(50))).expect("Internal code error"); //making sure we are bot held up
    let len = match socket.recv(&mut kgs_buf){
        Ok(n) => {
            n
        },
        Err(e) => { 
            println!("Error from key gen service : {}",e);
            return Err(String::from("We are facing some trouble on our side, please try again in a while."));
        }
    };

    let kgs_reply_obj : Job = match String::from_utf8(kgs_buf[0..len].to_vec()){
        Ok(kgs_reply) => { serde_json::from_str(&kgs_reply).expect("Error generated from serde in RKGS lib") },
        Err(e) => {
            println!("Error from kgs (returning invalif utf-8 chars) : {}",e);
            return Err(String::from("We are facing some trouble on our side, please try again in a while."));
        }
    };
    match kgs_reply_obj.work.as_str(){
        "checked" => {
            if kgs_reply_obj.error == "FOUND".to_string(){
                return Ok(true);
            }else{
                return Ok(false);
            }
        },
        _ => { let ret_err_string = match String::from_utf8(kgs_buf[0..len].to_vec()){
            Ok(original_err) => { original_err }
            Err(_) => {
                String::from("Error from KGS : Please try again in a while, we are facing issues on our side")
            }
        };
        return Err(ret_err_string)}
    }

}
