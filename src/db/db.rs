use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada en .env");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("No se pudo crear el pool de conexiones")
}
