// src/tcp_server.rs
use std::net::{SocketAddr, IpAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::PgConnection;
use std::collections::HashMap;
use crate::db::{update_robot_status, parse_robot_payload};
use std::time::{Instant, Duration};

pub type ActiveIps = Arc<Mutex<HashMap<IpAddr, std::time::Instant>>>;
pub type RobotIpMap = Arc<Mutex<HashMap<String, IpAddr>>>;
pub type RobotConnMap = Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>;

#[derive(Clone)]
pub struct TcpServer {
    pub tcp_port: u16,
    pub conn: Arc<Mutex<PgConnection>>,
    pub active_ips: ActiveIps,
    pub robot_ip_map: RobotIpMap,
    pub robot_conn_map: RobotConnMap, // 👈 新增
}

impl TcpServer {
    pub fn new(tcp_port: u16, conn: Arc<Mutex<PgConnection>>, active_ips: ActiveIps, robot_ip_map: RobotIpMap, robot_conn_map: RobotConnMap) -> Self {
        Self {
            tcp_port,
            conn,
            active_ips,
            robot_ip_map,
            robot_conn_map, // 👈 别忘了传入
        }
    }

    pub fn start(&self) {
        let port = self.tcp_port;
        let conn = self.conn.clone();
        let active_ips = self.active_ips.clone();
        let robot_ip_map = self.robot_ip_map.clone();
        let robot_conn_map = self.robot_conn_map.clone();

        // 启动清理任务
        start_ip_cleanup_task(active_ips.clone());

        tokio::spawn(async move {
            let addr = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(&addr)
                .await
                .expect(&format!("Failed to bind to {}", addr));

            println!("TCP server listening on {}", addr);

            loop {
                match listener.accept().await {
                    Ok((stream, peer_addr)) => {
                        println!("[TCP {}] Connection accepted from {}", port, peer_addr);
                        tokio::spawn(handle_connection(
                            stream,
                            peer_addr,
                            conn.clone(),
                            active_ips.clone(),
                            robot_ip_map.clone(),
                            robot_conn_map.clone()
                        ));
                    }
                    Err(e) => {
                        eprintln!("[TCP {}] Failed to accept connection: {}", port, e);
                    }
                }
            }
        });
    }
}

/// 5秒定时任务，清理30秒未活跃的IP
fn start_ip_cleanup_task(active_ips: ActiveIps) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let now = Instant::now();
            let mut map = active_ips.lock().await;
            let before_count = map.len();

            map.retain(|ip, &mut last_seen| {
                let keep = now.duration_since(last_seen) <= Duration::from_secs(5);
                if !keep {
                    println!("🧹 Removing inactive IP: {}", ip);
                }
                keep
            });

            let after_count = map.len();
            if before_count != after_count {
                println!("IP cleanup done. Active connections: {} -> {}", before_count, after_count);
            }
        }
    });
}

/// TCP连接处理函数
async fn handle_connection(
    mut stream: TcpStream,
    peer_addr: SocketAddr,
    conn: Arc<Mutex<PgConnection>>,
    active_ips: ActiveIps,
    robot_ip_map: RobotIpMap,
    robot_conn_map: RobotConnMap, 
){
    let mut buffer = [0u8; 1024];
    let ip = peer_addr.ip();
    let shared_stream = Arc::new(Mutex::new(stream)); // ✅ 封装后就可以 clone 了

    loop {
        let mut stream_guard = shared_stream.lock().await; // 🔓 加锁拿到可用 stream
        match stream_guard.read(&mut buffer).await {
            Ok(0) => {
                println!("[TCP] Client {} disconnected", peer_addr);
                break;
            }
            Ok(n) => {
                drop(stream_guard); // ✅ 提前释放锁，避免后续死锁

                let data = &buffer[..n];
                {
                    let mut map = active_ips.lock().await;
                    map.insert(ip, std::time::Instant::now());
                }

                if let Ok(text) = std::str::from_utf8(data) {
                    println!("[TCP] Received text: {}", text);

                    if let Some((id, elec, act)) = parse_robot_payload(text) {
                        {
                            let mut ip_map = robot_ip_map.lock().await;
                            ip_map.insert(id.clone(), peer_addr.ip());
                        }

                        {
                            let mut conn_map = robot_conn_map.lock().await;
                            conn_map.insert(id.clone(), shared_stream.clone()); // ✅ 不 move，clone Arc 即可
                        }

                        let mut conn_guard = conn.lock().await;
                        match update_robot_status(&mut conn_guard, &id, elec, act) {
                            Ok(_) => println!("✅ Updated robot {} in DB", id),
                            Err(e) => eprintln!("❌ Failed to update DB: {}", e),
                        }
                    }
                }

                // 回发原始数据
                let mut stream_guard = shared_stream.lock().await;
                if let Err(e) = stream_guard.write_all(data).await {
                    eprintln!("[TCP] Write error: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("[TCP] Read error: {}", e);
                break;
            }
        }
    }
}