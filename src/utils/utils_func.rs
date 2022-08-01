use hyper::{Body, Response, StatusCode};
use serde_json;

pub fn failed_status_response(error: String) -> Response<Body> {
    let error_json_string = format!(
        " {} \"status\" : \"false\" , \"error\" : \"{}\" {}",
        "{", error, "}"
    );
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", " application/json")
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
        .body(Body::from(response_string))
        .unwrap();
    return response;
}
