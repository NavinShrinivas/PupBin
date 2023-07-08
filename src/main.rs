mod entity;
mod routes;
mod utils;

extern crate dotenv;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use sea_orm::{ConnectOptions, Database};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main] //main async runtime
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //environment varibales :
    dotenv().ok();
    let database_name =
        env::var("DATABASE_NAME").expect("ERROR : DATABASE_NAME must be set in .env file!");
    let database_user =
        env::var("DATABASE_USER").expect("ERROR : DATABASE_USER must be set in .env file!");
    let database_pass =
        env::var("DATABASE_PASSWORD").expect("ERROR : DATABASE_PASSWORD must be set in .env file!");
    println!("Starting server...");
    println!("Storing to postgresql databse : {:#?}", database_name);
    //Postgresql Databse connection :
    let db_connection_url = format!(
        "mysql://{}:{}@localhost/{}",
        database_user, database_pass, database_name
    );
    let mut db_connection = ConnectOptions::new(db_connection_url);
    db_connection
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    let db_conn = Database::connect(db_connection).await.unwrap();
    let arc = Arc::new(db_conn);

    let api_addr = SocketAddr::from(([0, 0, 0, 0], 80));
    let router_service = make_service_fn(|_| {
        let inner_arc = Arc::clone(&arc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                routes::routes::router_function(req, Arc::clone(&inner_arc))
            }))
        }
    });

    let server = Server::bind(&api_addr).serve(router_service);
    println!("Server listening on port 80");
    match server.await {
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
    Ok(())
}
