use actix_web::{get, web::Data, HttpResponse};
use serde_json::json;

use crate::{error::Error, model::AppState};

#[get("/")]
pub async fn home(data: Data<AppState>) -> Result<HttpResponse, Error> {
    let users = &data
        .users
        .lock()
        .map_err(|_| Error::InternalError("Lock Error".to_string()))?;

    Ok(HttpResponse::Ok().json({
        json!({
            "data": users.to_vec(),
            "success": true
        })
    }))
}
