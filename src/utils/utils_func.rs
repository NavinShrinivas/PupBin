use hyper::{Response, Body, StatusCode};

pub fn failed_status_response(error : String) -> Response<Body>{
    let error_json_string = format!(" {} \"status\" : \"false\" , \"error\" : \"{}\" {}","{",error,"}");
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type" ," application/json")
        .body(Body::from(error_json_string)).unwrap();
    return response;
}
