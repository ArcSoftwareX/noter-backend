mod app;
mod constants;
mod init;
mod repo;
mod routes;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use app::{app_data::AppData, config::Config};
use init::cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init::init().unwrap();

    let config = Config::new().unwrap();
    let app_data = AppData::new(&config).await.unwrap();

    let app_data = web::Data::new(app_data);

    HttpServer::new(move || {
        App::new()
            .configure(init::configure)
            .app_data(app_data.clone())
            .wrap(init::build_session_mw(config.cookie_secret.as_bytes()))
            .wrap(Logger::default())
            .wrap(cors(config.app_uri.as_deref()))
    })
    .bind(init::get_server_address())?
    .run()
    .await
}
