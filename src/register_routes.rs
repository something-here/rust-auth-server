use actix::Addr;
use actix_web::web::{block, Data, Json, Path};
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

use crate::models::{DbExecutor, HandleRequest};
use crate::register_handler::{RegisterUser, UserData};

pub async fn register_user(
    invitation_id: Path<String>,
    user_data: Json<UserData>,
    db: Data<DbExecutor>,
) -> Result<HttpResponse, Error> {
    let msg = RegisterUser {
        // into_inner() returns the inner string value from Path
        invitation_id: invitation_id.into_inner(),
        password: user_data.password.clone(),
    };

    let db_response = block(move || db.handle(msg)).await;
    match db_response {
        Ok(slim_user) => Ok(HttpResponse::Ok().json(slim_user)),
        Err(service_error) => Ok(service_error.error_response()),
    }
}
