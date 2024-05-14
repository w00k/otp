use actix_web::{web, HttpResponse, Responder, post};
use chrono::Utc;

use crate::model::otp_keys::{DeleteByDateRequest, OtpMessageResponse};
use crate::query::delete::delete_by_date;
use crate::connection::connection;

#[post("/delete-by-date")]
pub async fn delete_otp_by_date(input_expiration_date: web::Json<DeleteByDateRequest>) -> impl Responder {
    log::info!("delete by expiration date {:?}", input_expiration_date);
    let expiration_date = input_expiration_date.into_inner();

    let pool = connection::establish_connection();
    delete_by_date(pool, expiration_date.expiration_date);

    let response = message_delete(expiration_date.expiration_date.to_string());
    HttpResponse::Ok().json(response)
}

fn message_delete(expiration_date_string: String) -> OtpMessageResponse {
    let naive_date_time = Utc::now().naive_utc();
    let message = format!("Delete by date:{}", expiration_date_string);
    let response = OtpMessageResponse {
        code: "OK".to_string(),
        message: message.to_string(),
        datetime: naive_date_time,
    };
    return response;
}