use diesel::{pg::PgConnection};

use crate::models::{
    otp_keys::{NewOtpKey, OtpKey},
};

pub fn new_otp_key(mut conn: PgConnection, new_otp_key: NewOtpḰey) {
    let result = OtpKey::create_otp_key(&mut conn, &new_otp_key);
    if result.is_ok() {
        println!("{:?}", result.unwrap());
    } else {
        println!("Error: new user {}\n", result.err().unwrap());
    }
}