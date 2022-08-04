
#[derive(serde_derive::Serialize)]
struct GetPasteRequest {
    paste_key: String,
}

pub async fn get(command_line_args: crate::CommandLineData, server_url: String) {
    let key = command_line_args.get_key;

    if key == "" {
        panic!("Please provide a key to get paste!");
    }

    let req_body = GetPasteRequest { paste_key: key };
    let req_body_json_string = match serde_urlencoded::to_string(&req_body){
        Ok(string) => { string },
        _ => {
            println!("Internal error : Can to form body for response");
            panic!("Serde error!")
        }
    };

    let client = reqwest::Client::new();
    let resp = match client
        .get(format!("{}{}", server_url, String::from("/getPaste")))
        .body(req_body_json_string)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            println!("Make sure you are connected to a network, if to report this to devs.");
            panic!("NETWORK ERROR")
        }
    };

    let resp_body_buff = match resp.bytes().await{
        Ok( bytes_vec ) => { bytes_vec },
        _ => {
            panic!("Internal error, error getting byte array from server")
        }
    };

    let resp_body_obj : serde_json::Value = match serde_json::de::from_slice(&resp_body_buff){
        Ok(hash_map) => {hash_map},
        Err(_) => { println!("Error deserialising response from server, possibly returning garbage or not utf-8 encoded"); panic!("Serde error") }
    };

    if resp_body_obj["status"] == true{
        if command_line_args.verbose == true{
            println!("Paste fetch succefully! \n Paste content :");
        }
        println!("{}", resp_body_obj["paste_content"].as_str().unwrap());
    }else{
        println!("Sorry, we couldnt get your paste.Make sure the given key is correct\nreponse :{}.", resp_body_obj);
    }

}
