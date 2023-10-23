use actix_web::get;

#[get("/healthcheck")]
pub async fn healthcheck() -> &'static str {
    "hello world"
}
