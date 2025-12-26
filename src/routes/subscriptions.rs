use actix_web::{HttpResponse, Responder, post, web};
use chrono::Utc;
use serde::{Deserialize};
use sqlx::{ PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
struct SubscriptionForm {
    email: String,
    name: String,
}

#[post("/subscription")]
async fn subscribe(
    form: web::Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
                    VALUES ($1, $2, $3, $4)
                    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to connect to execute the querry {}", e);
            HttpResponse::InternalServerError().finish()
        }
    };
    HttpResponse::Ok()
}
