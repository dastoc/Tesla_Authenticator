use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(database_url: &str) -> Result<DbPool, std::io::Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
}