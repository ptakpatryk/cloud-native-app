use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct SubsribeForm {
    name: String,
    email: String,
}

// #[post("/subscribe")]
pub async fn subscribe(
    form: web::Form<SubsribeForm>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;

    HttpResponse::Ok().finish()
}
