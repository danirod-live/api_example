mod endpoints;
mod state;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use log::info;
use state::AppState;
use std::sync::Mutex;

pub struct AppStateWithMutex {
    pub state: Mutex<AppState>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    let state = web::Data::new(AppStateWithMutex {
        state: Mutex::new(AppState::new()),
    });

    env_logger::init();
    info!("Lanzando el servidor web");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(endpoints::get_status)
            .service(endpoints::enqueue_new_task)
            .service(endpoints::get_info)
            .service(endpoints::get_result)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
