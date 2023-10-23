use actix_files as fs;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    dev::{fn_service, ServiceRequest, ServiceResponse},
    web,
};
use fs::NamedFile;

use crate::routes::{
    auth::{login, logout, signup},
    avatar::{get_avatar, upload_avatar},
    healthcheck::healthcheck,
    notes::{create_note, delete_note, get_notes, replace_note},
    users::me,
};

pub fn init() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv()?;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    Ok(())
}

pub fn get_server_address() -> (&'static str, u16) {
    (
        "0.0.0.0",
        std::env::var("PORT")
            .unwrap_or("8080".into())
            .parse::<u16>()
            .unwrap(),
    )
}

use actix_cors::Cors;

pub fn cors(app_uri: Option<&str>) -> Cors {
    Cors::default()
        .allow_any_header()
        .allowed_origin(app_uri.unwrap_or("http://localhost:5173"))
        .allowed_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .supports_credentials()
        .expose_any_header()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(healthcheck)
            .service(
                web::scope("/auth")
                    .service(signup)
                    .service(login)
                    .service(logout),
            )
            .service(web::scope("/users").service(me))
            .service(
                web::scope("/notes")
                    .service(get_notes)
                    .service(create_note)
                    .service(replace_note)
                    .service(delete_note),
            )
            .service(
                web::scope("/avatar")
                    .service(upload_avatar)
                    .service(get_avatar),
            ),
    )
    .default_service(
        fs::Files::new("/", "./public")
            .index_file("index.html")
            .show_files_listing()
            .default_handler(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let file = NamedFile::open_async("./public/index.html").await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            })),
    );
}

pub fn build_session_mw(secret: &[u8]) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(secret))
        .cookie_same_site(actix_web::cookie::SameSite::Lax)
        .cookie_domain(None)
        .cookie_secure(true)
        .cookie_http_only(true)
        .session_lifecycle(
            PersistentSession::default().session_ttl(Duration::seconds(4 * 7 * 24 * 60 * 60)), // 4 weeks, 7 days each, 24 hrs in a day, 60 mins in an hour, 60 secs in a minute
        )
        .build()
}
