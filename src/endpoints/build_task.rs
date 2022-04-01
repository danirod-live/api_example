use crate::state::Status;
use crate::AppStateWithMutex;
use actix_web::http::StatusCode;
use actix_web::rt as actix_rt;
use actix_web::web;
use actix_web::{post, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateTaskPayload {
    command: String, // El comando que vamos a correr
    #[serde(default)]
    priority: Option<i16>, // La prioridad del 1 al 5
}

#[derive(Serialize, Debug, Clone)]
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
        Err(e) => HttpResponse::build(StatusCode::BAD_REQUEST).body(e),
        Ok(p) => {
            let uuid = p.uuid.clone();
            let executed = p.command.to_uppercase();

            let mut state = data.state.lock().unwrap();
            state.put(&uuid, &p.command);
            drop(state);

            actix_rt::spawn(async move {
                info!("Vamos a esperarnos un poco");
                sleep(Duration::from_secs(15)).await;

                let mut database = data.state.lock().unwrap();
                database.update(&uuid, Status::Done(executed)).unwrap();
                info!("Han pasado 5 segundos");
            });
            HttpResponse::build(StatusCode::ACCEPTED).json(p)
        }
    }
}
