use actix_web::{HttpResponse, Responder, post, web};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriptionForm {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subcriber_name = %form.name
)
)]
#[post("/subscription")]
async fn subscribe(
    form: web::Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match insert_suscriber(&connection, &form).await {
        Ok(_) => {
            tracing::info!("Info of the new user have been saved ");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Could not save the info of the new user because : {:?}", e,);
            HttpResponse::InternalServerError().finish()
        }
    };
    HttpResponse::Ok()
}

#[tracing::instrument(name = "Start subscription querry", skip(pool, form))]
pub async fn insert_suscriber(pool: &PgPool, form: &SubscriptionForm) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
                    VALUES($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Could not subscribe bacause {}", e);
        e
    })?;
    Ok(())
}
