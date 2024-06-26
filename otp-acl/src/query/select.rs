use diesel::{pg::PgConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

use crate::model::otp_keys::{OtpKey};
use crate::model::otp_keys::OtpKeyRequest;

pub fn find_otp_key(mut conn: &mut PooledConnection<ConnectionManager<PgConnection>>, input_otp_key: OtpKeyRequest) -> Result<OtpKey, Error> {
    let result = OtpKey::select_otp_key(&mut conn, input_otp_key);

    if result.is_ok() {
        let msg_db = result.unwrap();
        let mut otp_key: OtpKey = OtpKey {
            id: 0,
            otp_public_key: "".to_string(),
            otp_private_key: "".to_string(),
            otp_user: "".to_string(),
            retry: 0,
            expiration_date: Default::default(),
            otp_key_enable: false,
        };

        println!("len {}", msg_db.len());

        for msg in msg_db {
            otp_key = msg;
        }

        Ok(otp_key)

    } else {
        let error = result.err().unwrap();
        println!("Error: new user {}", error);
        Err(error)
    }
}