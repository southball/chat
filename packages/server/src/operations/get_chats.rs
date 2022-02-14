use crate::schemas::{ChatMetadata, User, GUID};
use actix_web::{get, HttpResponse, Responder};

#[get("/chats")]
async fn get_chats() -> impl Responder {
    HttpResponse::Ok().json(vec![ChatMetadata {
        chat_guid: GUID("test".to_owned()),
        latest_message: "test".to_owned(),
        user: User {
            display_name: "test_dn".to_owned(),
            user_guid: GUID("test".to_owned()),
            username: "test".to_owned(),
        },
    }])
}
