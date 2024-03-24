use diesel::{pg::PgConnection};

use crate::model::{
    otp_keys::{OtpKey, NewOtpKey},
};

pub fn new_otp_key(mut conn: PgConnection, new_otp_key: NewOtpKey) {
    let result = OtpKey::create_otp_key(&mut conn, &new_otp_key);
    if result.is_ok() {
        let msg_db = result.unwrap();
        println!("{} - {} - {} - {} - {} - {}", msg_db.id, msg_db.otp_public_key, msg_db.otp_user, msg_db.retry, msg_db.expiration_date, msg_db.otp_key_enable);
    } else {
        println!("Error: new user {}\n", result.err().unwrap());
    }
}