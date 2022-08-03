mod actions;
mod utils;

pub struct CommandLineData {
    file_path: String,
    lifetime: String,
    paste: bool,
    get: bool,
    get_key: String,
}

impl CommandLineData {
    fn new() -> CommandLineData {
        CommandLineData {
            file_path: String::from(""),
            lifetime: String::from("0030"),
            paste: false,
            get: false,
            get_key: String::from(""),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_line_args = utils::commandline_utils::commandline_processer();
    let backend_server_url = String::from("http://0.0.0.0:5000");
    if command_line_args.paste == true {
        actions::paste::paste(command_line_args, backend_server_url).await;
    } else if command_line_args.get == true {
        actions::get::get(command_line_args, backend_server_url).await;
    }

    Ok(())
}
