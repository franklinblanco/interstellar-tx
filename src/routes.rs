use actix_web::{HttpResponse, post, web::Path};
use uuid::Uuid;

use crate::queue;

#[post("/itx/{id}")]
pub async fn commit_itx(id: Path<Uuid>) -> HttpResponse {
    match queue::commit(&id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Error committing itx: {error}");
            HttpResponse::NotFound().finish()
        },
    };
    HttpResponse::Ok().finish()
}
#[post("/itx/{id}")]
pub async fn abort_itx(id: Path<Uuid>) -> HttpResponse {
    match queue::abort(&id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Error aborting itx: {error}");
            HttpResponse::NotFound().finish()
        },
    };
    HttpResponse::Ok().finish()
}

