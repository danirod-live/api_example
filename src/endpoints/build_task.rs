use crate::AppStateWithMutex;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::{post, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateTaskPayload {
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

    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    Ok(CreatedTask {
        command: t.command.clone(),
        priority: final_priority,
        uuid,
    })
}

#[post("/")]
pub async fn enqueue_new_task(
    data: web::Data<AppStateWithMutex>,
    task: web::Json<CreateTaskPayload>,
) -> impl Responder {
    info!("Han llamado al encolado {:?}", &task);
    let payload = build_task(&task);
    match payload {
        Ok(p) => {
            let mut state = data.state.lock().unwrap();
            state.put(&p.uuid, &p.command);
            HttpResponse::build(StatusCode::ACCEPTED).json(p)
        }
        Err(e) => HttpResponse::build(StatusCode::BAD_REQUEST).body(e),
    }
}
