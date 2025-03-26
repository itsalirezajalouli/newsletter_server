use actix_web::{HttpResponse, Responder};

//  A type implements the Responder trait if it can be converted
//  into a HttpResponse, actix_web automatically converts it to HttpResponse
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
