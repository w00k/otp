use actix_web::{web, HttpResponse, Responder, post};
use chrono::Utc;

use crate::model::otp_keys::{DeleteByDateRequest, OtpMessageResponse};
use crate::query::delete::delete_by_date;
use crate::controller::create_otp::DbPool;

#[utoipa::path(
    request_body = DeleteByDateRequest,
    responses(
        (status = 200, description = "Some response OK", body = [OtpMessageResponse])
    )
)]
#[post("/delete-by-date")]
pub async fn delete_otp_by_date(pool:  web::Data<DbPool>, input_expiration_date: web::Json<DeleteByDateRequest>) -> impl Responder {
    log::info!("delete by expiration date {:?}", input_expiration_date);
    let expiration_date = input_expiration_date.into_inner();

    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    delete_by_date(&mut conn, expiration_date.expiration_date);

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