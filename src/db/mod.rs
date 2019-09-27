use log::*;
use r2d2::{Pool, ManageConnection, PooledConnection};
use postgres::{Connection, Error, TlsMode};
use super::util::config::*;
use super::service::user::{user_exists, user_create};
use super::model::user::CreatePath::*;

lazy_static! {
    static ref POOL: Pool<Manager> = create_pool();
}

fn create_pool() -> Pool<Manager> {
    let config = get_config();
    let manager = Manager::new(
        config.get(DB_HOST),
        config.get(DB_PORT),
        config.get(DB_NAME),
        config.get(DB_USERNAME),
        config.get(DB_PASSWORD),
    );

    let pool = Pool::builder()
        .min_idle(Some(5))
        .max_size(15)
        .build(manager)
        .unwrap();

    debug!("Create database pool");
    
    pool
}

pub fn build_datasource() {
    let config = get_config();

    let conn = &*get_connection();
    let trans = conn.transaction().unwrap();
    info!("initialize database...");
    //create admin user
    let username = config.get(BUILD_ADMIN_USERNAME);
    let password = config.get(BUILD_ADMIN_PASSWORD);
    let name = config.get(BUILD_ADMIN_NAME);
    if !user_exists(&trans, &username) {
        if let Err(e) = user_create(&trans, &username, &password, &name, true, System) {
            error!("User create failed: {}", e);
        }
    }else{
        warn!("user {} is already exists.", username);
    }
    //create global setting

    trans.commit().unwrap();
    info!("Done.")
}

pub fn get_connection() -> PooledConnection<Manager> {
    POOL.get().unwrap()
}

pub struct Manager {
    url: String
}
impl Manager {
    fn new(host: &str, port: &str, name: &str, username: &str, password: &str) -> Self {
        Self {
            url: format!("postgresql://{}:{}@{}:{}/{}", username, password, host, port, name)
        }
    }
}
impl ManageConnection for Manager {
    type Connection = Connection;
    type Error = Error;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Connection::connect(self.url.clone(), TlsMode::None)
    }
    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        if let Err(e) = conn.query("SELECT 1", &[]) {
            Err(e)
        }else{
            Ok(())
        }
    }
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}