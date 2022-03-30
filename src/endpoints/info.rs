use crate::AppStateWithMutex;
use actix_web::{get, web, HttpResponse};

#[get("/{id}/status")]
pub async fn get_info(
    task_id: web::Path<String>,
    data: web::Data<AppStateWithMutex>,
) -> HttpResponse {
    let state = data.state.lock().unwrap();
    match state.get(&task_id) {
        None => HttpResponse::NotFound().body("NO SE DE QUIEN ME HABLA USTED"),
        Some(ref command) => {
            let response = format!("Soy la tarea {} y mi comando es {}", task_id, command);
            HttpResponse::Ok().body(response)
        }
    }
}
