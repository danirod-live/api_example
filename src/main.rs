use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct CreateTaskPayload {
    command: String, // El comando que vamos a correr
    #[serde(default)]
    priority: Option<i16>, // La prioridad del 1 al 5
}

#[derive(Serialize, Debug)]
struct CreatedTask {
    command: String,
    priority: i16,
    uuid: String,
}

fn build_task(t: &CreateTaskPayload) -> Result<CreatedTask, &'static str> {
    let final_priority = t.priority.unwrap_or(3);
    if !(1..=5).contains(&final_priority) {
        return Err("Invalid priority");
    }
    Ok(CreatedTask {
        command: t.command.clone(),
        priority: final_priority,
        uuid: String::from("3019482730948127304"),
    })
}

#[post("/")]
async fn enqueue_new_task(task: web::Json<CreateTaskPayload>) -> impl Responder {
    info!("Han llamado al encolado {:?}", &task);
    let payload = build_task(&task);
    match payload {
        Ok(p) => HttpResponse::build(StatusCode::ACCEPTED).json(p),
        Err(e) => HttpResponse::build(StatusCode::BAD_REQUEST).body(e),
    }
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("ESTAMOS O NO ESTAMOS")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();
    info!("Lanzando el servidor web");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(status)
            .service(enqueue_new_task)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
