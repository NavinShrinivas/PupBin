extern crate tokio;

mod routes;
mod utils;

use dotenv::dotenv;
use hyper::{service::make_service_fn, service::service_fn, Server};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
use std::{convert::Infallible, env, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //From dotenv docs :
    //This is usually what you want. It loads the .env file located in the environment's current directory or its parents in sequence.
    // Meaning it can take the env file from parent.

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
        "postgres://{}:{}@localhost/{}",
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

    let api_addr = SocketAddr::from(([0, 0, 0, 0], 5002));
    let router_service = make_service_fn(|_| {
        let inner_arc = Arc::clone(&arc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                routes::routes::router_function(req, Arc::clone(&inner_arc))
            }))
        }
    });

    let server = Server::bind(&api_addr).serve(router_service);
    println!("Cleanup Server listening on port 5002");
    match server.await {
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
    Ok(())
}
