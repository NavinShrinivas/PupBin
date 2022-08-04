use hyper::{Body, Response, StatusCode};
use serde_json;

pub fn failed_status_response(error: String) -> Response<Body> {
    let error_json_string = format!(" {{ \"status\" : \"false\" , \"error\" : \"{}\" }}", error);
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", " application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(error_json_string))
        .unwrap();
    return response;
}

pub fn success_status_response<T: serde::ser::Serialize>(response_struct: T) -> Response<Body> {
    let response_string = match serde_json::to_string(&response_struct) {
        Ok(string) => string,
        Err(_) => return failed_status_response(String::from("INTERNALERROR")),
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", " application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(response_string))
        .unwrap();
    return response;
}

pub fn preflight_response() -> Response<Body> {
    println!("Preflight invoked!");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(Body::default())
        .unwrap();
    response
}

pub fn install_script() -> Response<Body> {
    let script = String::from(
        "
echo 'Hello world, this is the pupbin install script'
echo 'Installing pupbin tool...'
echo 'cloning repo'
git clone --depth=1 git@github.com:NavinShrinivas/PupBin.git ~/PupBinSources
if [ $? -ne \"0\" ];then
    echo 'build failed'
    exit
fi
cd ~/PupBinSources
echo 'Checking if rust toolchains are present...'
cargo > /dev/null
if [ $? -ne \"0\" ];then
    echo \"Rust toolchains not found, installing rust\"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
    echo \"Rust toolchains found!\"
fi
if [ $? -ne \"0\" ];then
    echo 'build failed'
    exit
fi
echo \"building tool...\"
cd Frontend
cargo build --release
if [ $? -ne \"0\" ];then
    echo 'build failed'
    exit
fi
if [ $? -eq \"0\" ];then
    sudo cp ./target/debug/pupbin /bin
fi

");
    return Response::new(Body::from(script));
}
