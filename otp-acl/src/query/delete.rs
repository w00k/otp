use diesel::pg::PgConnection;
use diesel::internal::derives::multiconnection::chrono;

use crate::model::otp_keys::OtpKey;

pub fn delete_by_id(mut conn: PgConnection, delete_id: i32) {
    let rows = OtpKey::delete_by_id(&mut conn, delete_id);
    println!("rows {}", rows);
}

pub fn delete_by_date(mut conn: PgConnection, input_expiration_date: chrono::NaiveDateTime) {
    let query_result = OtpKey::delete_by_date(&mut conn, input_expiration_date);
    println!("{:?}", query_result);
}