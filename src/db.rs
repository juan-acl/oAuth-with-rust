use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::thread;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATADASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // Creacion de 10 hilos
    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();

            assert!(connection.is_ok());
        });
    }
}
