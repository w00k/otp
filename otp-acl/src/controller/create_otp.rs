use actix_web::{web, HttpResponse, Responder, put};
use chrono::Utc;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

use crate::model::otp_keys::{NewOtpKey, OtpKeyResponse, OtpMessageResponse};
use crate::query::insert::new_otp_key;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[utoipa::path(
    request_body = NewOtpKey,
    responses(
        (status = 201, description = "Create OTP Key", body = [OtpKeyResponse]),
        (status = 200, description = "Some error message", body = [OtpMessageResponse])
    )
)]
#[put("/create")]
pub async fn create_otp_key(pool:  web::Data<DbPool>, otp: web::Json<NewOtpKey>) -> impl Responder {
    log::info!("create otp {:?}", otp);
    let otp_key_request: NewOtpKey = otp.into_inner();

    //let pool = connection::establish_connection();
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    let otp = new_otp_key(&mut conn, otp_key_request);
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