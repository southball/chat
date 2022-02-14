use crate::schemas::{ChatMetadata, User, GUID};
use actix_web::{get, web::Data, web::Path, HttpResponse, Responder};
use sqlx::PgPool;
use std::sync::Arc;

#[get("/chat/{chat_guid}")]
async fn get_chat(db: Data<Arc<PgPool>>, path: Path<(GUID,)>) -> impl Responder {
    let (chat_guid,) = path.into_inner();
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM Accounts")
        .fetch_one(&***db)
        .await
        .unwrap();
    HttpResponse::Ok().json(ChatMetadata {
        chat_guid,
        latest_message: count.to_string(),
        user: User {
            display_name: "test".to_owned(),
            user_guid: GUID("test".to_owned()),
            username: "test".to_owned(),
        },
    })
}
