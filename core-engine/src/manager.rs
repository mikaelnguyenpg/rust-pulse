use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::monitor::{DataProvider, RealTimeProvider};

pub struct PulseManager {
    // Dùng Arc<RwLock> để có thể chia sẻ Provider giữa các Task nếu cần
    provider: Arc<RwLock<RealTimeProvider>>,
    // Kênh phát tin: Cho phép nhiều UI cùng lắng nghe một nguồn dữ liệu
    tx: broadcast::Sender<String>,
}

impl PulseManager {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            provider: Arc::new(RwLock::new(RealTimeProvider::new())),
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub async fn start_loop(&self, interval_ms: u64) {
        let tx = self.tx.clone();
        let provider = self.provider.clone();

        tokio::spawn(async move {
            let interval = Duration::from_millis(interval_ms);
            loop {
                // Mở scope để đảm bảo Lock Guard 'p' được drop ngay sau khi lấy data                
                let json_data = {
                    let mut p = provider.write().await;
                    let data = p.fetch_data().await;
                    serde_json::to_string(&data).ok()
                };

                if let Some(json) = json_data {
                    // Gửi dữ liệu tới tất cả các subscribers (React, Leptos, v.v.)
                    let _ = tx.send(json);
                }

                sleep(interval).await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pulse_manager_flow() {
        // 1. Khởi tạo manager với buffer 16 tin nhắn
        let manager = PulseManager::new(16);
        let mut receiver = manager.subscribe();

        // 2. Chạy vòng lặp với tốc độ nhanh (100ms) để test
        manager.start_loop(100).await;

        // 3. Chờ đợi nhận tin nhắn đầu tiên
        let received = receiver.recv().await;

        assert!(received.is_ok(), "Phải nhận được dữ liệu từ broadcast channel");
        let json_str = received.unwrap();

        // 4. Kiểm tra xem dữ liệu có phải là JSON hợp lệ không
        assert!(json_str.contains("total_cpu"), "Dữ liệu JSON phải chứa thông tin CPU");
        assert!(json_str.contains("processes"), "Dữ liệu JSON phải chứa danh sách tiến trình");

        println!(" *** Successfully received pulse: {} bytes", json_str.len());
    }
}
