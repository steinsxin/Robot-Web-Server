use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::services::db::DbPool;
use std::net::IpAddr;
use std::time::Instant;
use tokio::net::TcpStream;

pub type RobotIpMap = Arc<Mutex<HashMap<String, IpAddr>>>;
pub type RobotConnMap = Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>;
pub type ActiveIps = Arc<Mutex<HashMap<String, Instant>>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub active_ips: ActiveIps,
    pub robot_ip_map: RobotIpMap,
    pub robot_conn_map: RobotConnMap,
}

impl AppState {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool,
            active_ips: Arc::new(Mutex::new(HashMap::new())),
            robot_ip_map: Arc::new(Mutex::new(HashMap::new())),
            robot_conn_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}