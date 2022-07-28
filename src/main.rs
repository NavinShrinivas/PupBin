mod routes;
mod utils;



use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::SocketAddr;



#[tokio::main] //main async runtime
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let api_addr = SocketAddr::from(([0,0,0,0],5000));
    let router_service = make_service_fn(|_| {
        async{ Ok::<_,Infallible>(service_fn(routes::routes::router_function)) }
    });

    let server = Server::bind(&api_addr).serve(router_service);
    match server.await{
        Err(e) =>{
            println!("{}",e);
        },
        _ => {

        }
    }
    Ok(())

}
