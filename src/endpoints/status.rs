use crate::AppStateWithMutex;
use actix_web::web;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get_status(data: web::Data<AppStateWithMutex>) -> impl Responder {
    let state = data.state.lock().unwrap();
    let output = format!("ESTAMOS: {} tareas encoladas\n", state.length());
    HttpResponse::Ok().body(output)
}
