use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PoolPg = Pool<ConnectionManager<PgConnection>>;
pub type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;


pub fn pg_pool() -> PoolPg {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    //Pool::new(manager).expect("Postgres connection pool could not be created");
    Pool::builder()
        .max_size(25)
        .build(manager).expect("Postgres connection pool could not be created")


}