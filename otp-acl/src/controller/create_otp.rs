use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::model::otp_keys::{NewOtpKey, OtpKeyResponse, OtpMessageResponse};
use crate::query::insert::new_otp_key;
use crate::connection::connection;

pub async fn create_otp_key(otp: web::Json<NewOtpKey>) -> impl Responder {
    log::info!("create otp {:?}", otp);
    let otp_key_request: NewOtpKey = otp.into_inner();

    let pool = connection::establish_connection();

    let otp = new_otp_key(pool, otp_key_request);
    if otp.is_ok() {
        let otp_key = otp.unwrap();
        let response = OtpKeyResponse {
            otp_public_key: otp_key.otp_public_key,
            otp_user: otp_key.otp_user,
            retry: otp_key.retry,
            otp_key_enable: otp_key.otp_key_enable,
        };
        HttpResponse::Created().json(response)
    } else {
        let response = message_error();
        HttpResponse::Ok().json(response)
    }
}

fn message_error() -> OtpMessageResponse {
    let naive_date_time = Utc::now().naive_utc();
    let response = OtpMessageResponse {
        code: "NOT VALID".to_string(),
        message: "OTP is not valid".to_string(),
        datetime: naive_date_time,
    };
    return response;
}