use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde;

// When Actix Web receives the request, it tries to deserialize the form data
// into your FormData struct, so if they don't match the framework rejects it
// before it even reaches your handler function.
#[derive(serde::Deserialize)]
struct FormData {
    username: String,
    email: String,
}

//  A type implements the Responder trait if it can be converted
//  into a HttpResponse, actix_web automatically converts it to HttpResponse
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer: handles transport level concerns (listens for requests)
    let server = HttpServer::new(|| {            
        // App: All the logic from getting the request to spitting a response
        App::new()
            // we call these endpoints
            .route("/health_check", web::get().to(health_check)) 
            .route("/subscriptions", web::post().to(subscribe)) 
    })
    .listen(listener)?
    .run();
    // .await    this will result in never ending and listening to server indefinetly
    Ok(server)
}
