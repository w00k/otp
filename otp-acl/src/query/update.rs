use diesel::{pg::PgConnection};

use crate::model::{
    otp_keys::{OtpKey},
};

pub fn update_retry(mut conn: PgConnection, update_id: i32, update_retry: i32) {
    let query_result = OtpKey::update_retry(&mut conn, update_id, update_retry);
    println!("{:?}", query_result);
}