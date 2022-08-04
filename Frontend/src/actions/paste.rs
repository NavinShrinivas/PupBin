use serde_json;
use serde_urlencoded;
use std::fs;
use std::path::Path;

#[derive(serde_derive::Serialize)]
struct NewPasteRequest {
    paste_data: String,
    lifetime: String,
}

pub async fn paste(command_line_args: crate::CommandLineData, server_url: String) {
    //first check if file exists
    let paste_data = match Path::new(&command_line_args.file_path).is_file() {
        true => {
            println!("Pasting content of : {}", command_line_args.file_path);
            fs::read_to_string(command_line_args.file_path)
                .expect("Something went wrong reading the file")
        }
        _ => {
            panic!("Please provide a valid file!")
        }
    };

    let body_obj = NewPasteRequest {
        paste_data,
        lifetime: command_line_args.lifetime,
    };
    let body = match serde_urlencoded::ser::to_string(body_obj) {
        Ok(body) => body,
        Err(_) => {
            panic!("Error in clinet side!!!! Report to developers")
        }
    };
    let client = reqwest::Client::new();
    let resp = match client
        .post(&format!("{}{}", &server_url, &String::from("/newPaste")))
        .body(body)
        .send()
        .await
        {
            Ok(resp) => resp,
            _ => {
                panic!("Network error! Make sure you are connected. If so, report to devs")
            }
        };
    let resp_body_buffer = match resp.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(_) => {
            panic!("Network error. Erred recving response.")
        }
    };
    let resp_obj: serde_json::Value = match serde_json::from_slice(&resp_body_buffer) {
        Ok(obj) => obj,
        _ => {
            panic!("Backend is returning garbage, try again in a while!")
        }
    };

    if resp_obj["status"] == true {
        if command_line_args.verbose == true{
            println!("Paste made successfully!");
            println!(
                "Paste link : example.com/{} [Note : Web frontend not created yet :( ]",
                resp_obj["key"]
                );
        }
        println!("Paste key : {}", resp_obj["key"]);

    } else {
        println!("Failed to make paste :(");
        println!("Reponse : {}", resp_obj);
    }
}
