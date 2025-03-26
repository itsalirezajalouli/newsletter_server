use serde;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::routes::{health_check, subscribe};
use sqlx::PgConnection;

// When Actix Web receives the request, it tries to deserialize the form data
// into your FormData struct, so if they don't match the framework rejects it
// before it even reaches your handler function.
#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

pub fn run(listener: TcpListener,
           connection: PgConnection) -> Result<Server, std::io::Error> {
    // HttpServer: handles transport level concerns (listens for requests)
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {            
        // App: All the logic from getting the request to spitting a response
        App::new()
            // we call these endpoints
            .route("/health_check", web::get().to(health_check)) 
            .route("/subscriptions", web::post().to(subscribe)) 
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    // .await    this will result in never ending and listening to server indefinetly
    Ok(server)
}
