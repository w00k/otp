use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::model::otp_keys::{OtpKeyRequest, OtpMessageResponse};
use crate::query::update::update_retry;
use crate::query::delete::delete_by_id;
use crate::query::select::find_otp_key;
use crate::connection::connection;

pub async fn validate_otp_key(otp: web::Json<OtpKeyRequest>) -> impl Responder {
    log::info!("validate otp {:?}", otp);
    let otp_key_request: OtpKeyRequest = otp.into_inner();
    let pool = connection::establish_connection();
    let otp_key_response = find_otp_key(pool, otp_key_request.clone());
    
    if otp_key_response.is_ok() {
        let pool = connection::establish_connection();
        let otp_key = otp_key_response.unwrap();

        if otp_key.otp_private_key == otp_key_request.otp_private_key && otp_key.retry > 0 {
            let response = message_ok();
            delete_by_id(pool, otp_key.id);
            HttpResponse::Ok().json(response)   
        } else { 
            if otp_key.otp_private_key != otp_key_request.otp_private_key && otp_key.retry > 0 {
                let pool = connection::establish_connection();
                let retry = otp_key.retry - 1;
                update_retry(pool, otp_key.id, retry); 
            } else {
                let pool = connection::establish_connection();
                delete_by_id(pool, otp_key.id);
            }
            let response = message_not_ok();
            HttpResponse::Ok().json(response)  
        }
    } else {
        HttpResponse::NotFound().json("ERROR")
    }
}

fn message_ok() -> OtpMessageResponse {
    // chrono::NaiveDateTime;
    let naive_date_time = Utc::now().naive_utc();
    let response = OtpMessageResponse {
        code: "OK".to_string(),
        message: "OTP is valid".to_string(),
        datetime: naive_date_time,
    };
    return response;
}

fn message_not_ok() -> OtpMessageResponse {
    // chrono::NaiveDateTime;
    let naive_date_time = Utc::now().naive_utc();
    let response = OtpMessageResponse {
        code: "NOT VALID".to_string(),
        message: "OTP is not valid".to_string(),
        datetime: naive_date_time,
    };
    return response;
}