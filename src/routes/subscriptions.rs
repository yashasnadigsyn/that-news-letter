use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::{chrono, uuid};
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let res_sqlx = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(pool.get_ref())
        .await;

    match res_sqlx {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query!\n ERROR: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}