use uuid::Uuid as UuidGen;
use chrono::Utc;
use sqlx::PgPool; 
use sqlx::types::Uuid; 
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// here Form calls Form::from_request to deserialize our request to FormData
// with serde and if it fails returns a 400 BAD REQUEST
pub async fn subscribe(form: web::Form<FormData>,
                       pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::from_u128(UuidGen::new_v4().as_u128()),
            form.email,
            form.name,
            Utc::now()
            )
            .execute(pool.get_ref())
            .await {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => {
                    println!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
}

