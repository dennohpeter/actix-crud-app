use std::{env, sync::Mutex};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;
use identity::{
    model::AppState,
    service::{home::home, login::login, register::register, status::status},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // global state

    let state = Data::new(AppState {
        app_name: format!("{} App", env!("CARGO_PKG_NAME")),
        users: Mutex::new(Vec::new()),
    });

    let factory = move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(home)
            .service(register)
            .service(login)
            .service(status)
    };

    HttpServer::new(factory)
        .bind(("127.0.0.1", 5000))? // bind the server to the address
        .run()
        .await
}
