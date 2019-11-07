use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PostgresPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PostgresPool {
    let database_url = dotenv!("DATABASE_URL");
    init_pool(&database_url).expect(&format!("[ERROR]: Could not connect to {}", database_url))
}
