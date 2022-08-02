use std::env;
use std::path::Path;
use serde_urlencoded;

#[derive(serde_derive::Serialize)]
struct NewPasteRequest{
    paste_data : String,
    lifetime : String
}



#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let file_path = match env::args().nth(1){
        Some(path) => {path}
        None => { panic!("Please provide a file to paste content") }
    };

    //first check if file exists
    match Path::new(&file_path).is_file(){
        true =>{
            println!("Pasting content of : {}", file_path);
        },
        _ => { panic!("Please provide a valid file!") }
    }
    let body_obj = NewPasteRequest{
        paste_data : String::from("test"),
        lifetime : String::from("0040")
    };
    let body = serde_urlencoded::ser::to_string(body_obj).unwrap();
    let client = reqwest::Client::new();
    let resp = client.post("http://0.0.0.0:5000/newPaste")
        .body(body)
        .send()
        .await?;
    println!("{}", String::from_utf8(resp.bytes().await?.to_vec()).unwrap());
    Ok(())

    }
