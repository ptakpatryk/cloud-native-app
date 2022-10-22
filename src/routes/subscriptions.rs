use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SubsribeForm {
    name: String,
    email: String,
}

// #[post("/subscribe")]
pub async fn subscribe(form: web::Form<SubsribeForm>) -> HttpResponse {
    println!("--------------------------------------------------------------------");
    println!("{:?}", form);

    HttpResponse::Ok().finish()
}
