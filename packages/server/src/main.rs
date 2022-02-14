mod operations;
mod schemas;

use actix_cors::Cors;
use actix_web::{error, web::{Data, PathConfig}, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());

    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080");
        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(PathConfig::default().error_handler(|err, _req| {
                let body = match err {
                    error::PathError::Deserialize(ref err) => format!("{}", err),
                    _ => "Unknown error when parsing path.".to_owned()
                };
                error::InternalError::from_response(err, HttpResponse::BadRequest().body(body))
                    .into()
            }))
            .service(operations::get_chats::get_chats)
            .service(operations::get_chat::get_chat)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
