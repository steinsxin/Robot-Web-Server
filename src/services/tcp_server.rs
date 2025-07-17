use std::net::{SocketAddr, IpAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use crate::state::app_state::AppState;
use crate::services::db::{parse_robot_payload, update_robot_status};

pub struct TcpServer {
    port: u16,
    state: Arc<AppState>,
}

impl TcpServer {
    pub fn new(port: u16, state: Arc<AppState>) -> Self {
        Self { port, state }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("TCP server listening on port {}", self.port);
        
        // 启动清理任务
        tokio::spawn(ip_cleanup_task(self.state.clone()));
        
        loop {
            let (stream, addr) = listener.accept().await?;
            let state = self.state.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, state).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn ip_cleanup_task(state: Arc<AppState>) {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let now = Instant::now();
        let mut map = state.active_ips.lock().await;
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
}

async fn handle_connection(
    mut stream: TcpStream,
    peer_addr: SocketAddr,
    state: Arc<AppState>,
) -> anyhow::Result<()> {
    let mut buffer = [0u8; 1024];
    let ip = peer_addr.ip();
    let shared_stream = Arc::new(Mutex::new(stream));

    loop {
        let mut stream_guard = shared_stream.lock().await;
        match stream_guard.read(&mut buffer).await {
            Ok(0) => {
                println!("[TCP] Client {} disconnected", peer_addr);
                break;
            }
            Ok(n) => {
                drop(stream_guard);

                let data = &buffer[..n];
                {
                    let mut map = state.active_ips.lock().await;
                    map.insert(ip.to_string(), Instant::now());
                }

                if let Ok(text) = std::str::from_utf8(data) {
                    println!("[TCP] Received text: {}", text);

                    if let Some((id, elec, act)) = parse_robot_payload(text) {
                        {
                            let mut ip_map = state.robot_ip_map.lock().await;
                            ip_map.insert(id.clone(), peer_addr.ip());
                        }

                        {
                            let mut conn_map = state.robot_conn_map.lock().await;
                            conn_map.insert(id.clone(), shared_stream.clone());
                        }

                        let mut conn = state.db_pool.get()?;
                        match update_robot_status(&mut conn, &id, elec, act) {
                            Ok(_) => println!("✅ Updated robot {} in DB", id),
                            Err(e) => eprintln!("❌ Failed to update DB: {}", e),
                        }
                    }
                }

                let mut stream_guard = shared_stream.lock().await;
                stream_guard.write_all(data).await?;
            }
            Err(e) => {
                eprintln!("[TCP] Read error: {}", e);
                break;
            }
        }
    }

    Ok(())
}