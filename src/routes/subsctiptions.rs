use uuid::Uuid;
use chrono::Utc;
use sqlx::PgConnection;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// here Form calls Form::from_request to deserialize our request to FormData
// with serde and if it fails returns a 400 BAD REQUEST
pub async fn subscribe(form: web::Form<FormData>,
                       connection: web::Data<PgConnection>) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
        )
        .execute(connection.get_ref())
        .await;

    HttpResponse::Ok().finish()
}


