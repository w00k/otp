use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

use crate::model::otp_keys::{OtpKey, NewOtpKey};

pub fn new_otp_key(mut conn: &mut PooledConnection<ConnectionManager<PgConnection>>, new_otp_key: NewOtpKey) -> Result<OtpKey, Error> {
    let result = OtpKey::create_otp_key(&mut conn, &new_otp_key);
    return result;
}