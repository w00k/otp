use diesel::{pg::PgConnection};

use crate::model::{
    otp_keys::{OtpKey},
};
use crate::model::otp_keys::OtpKeyRequest;

pub fn find_otp_key(mut conn: PgConnection, input_otp_key: OtpKeyRequest) {
    let result = OtpKey::select_otp_key(&mut conn, input_otp_key);
    if result.is_ok() {
        let msg_db = result.unwrap();
        println!("len {}", msg_db.len());

        for msg in msg_db {
            println!("{} - {} - {} - {} - {} - {} - {}", msg.id, msg.otp_public_key, msg.otp_private_key, msg.otp_user, msg.retry, msg.expiration_date, msg.otp_key_enable);
        }

    } else {
        println!("Error: new user {}\n", result.err().unwrap());
    }
}