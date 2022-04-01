use crate::state::Status;
use crate::AppStateWithMutex;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ResultResponse<'a> {
    result: &'a str,
    outcome: bool,
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
    outcome: bool,
}

#[get("/{id}/result")]
pub async fn get_result(
    task_id: web::Path<String>,
    data: web::Data<AppStateWithMutex>,
) -> impl Responder {
    let state = data.state.lock().unwrap();

    let task = state.get(&task_id);
    if task.is_none() {
        return HttpResponse::NotFound().body("");
    }

    match task.unwrap().status {
        Status::Done(ref res) => {
            let result = ResultResponse {
                result: &res,
                outcome: true,
            };
            HttpResponse::build(StatusCode::OK).json(result)
        }
        Status::Error(ref e) => {
            let result = ErrorResponse {
                error: &e,
                outcome: false,
            };
            HttpResponse::build(StatusCode::OK).json(result)
        }
        _ => HttpResponse::build(StatusCode::BAD_REQUEST).body("No procesable"),
    }
}
