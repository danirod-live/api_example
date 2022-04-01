use crate::state::{Status, Task};
use crate::AppStateWithMutex;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct InfoResponse<'a> {
    uuid: &'a str,
    command: &'a str,
    status: &'a Status,
}

#[get("/{id}/status")]
pub async fn get_info(
    task_id: web::Path<String>,
    data: web::Data<AppStateWithMutex>,
) -> impl Responder {
    let state = data.state.lock().unwrap();
    match state.get(&task_id) {
        None => HttpResponse::NotFound().body(""),
        Some(ref task) => {
            let response = extract_info(&task_id, task);
            HttpResponse::build(StatusCode::OK).json(response)
        }
    }
}

fn extract_info<'a>(uuid: &'a str, t: &'a Task) -> InfoResponse<'a> {
    InfoResponse {
        uuid,
        command: &t.command,
        status: &t.status,
    }
}
