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
    println!("Cleanup Preflight invoked!");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "http://0.0.0.0:80")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(Body::default())
        .unwrap();
    response
}
