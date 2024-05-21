use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    // read DATABASE_URL from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("No se puedo construir el pool de conexiones");
    return pool;
}