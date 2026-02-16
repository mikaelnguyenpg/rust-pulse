use crate::models::{SystemPulse, ProcessPulse};
use async_trait::async_trait;
use std::borrow::Cow;
use sysinfo::System;

// --- PHẦN 1: ĐỊNH NGHĨA GIAO DIỆN (TRAIT) ---
// Giống như một bản hợp đồng: "Bất cứ ai là DataProvider thì phải có hàm fetch_data"
#[async_trait]
pub trait DataProvider: Send + Sync {
    // Trả về SystemPulse với Lifetime gắn liền với chính Provider
    async fn fetch_data(&mut self) -> SystemPulse<'_>;
}

// --- PHẦN 2: CÀI ĐẶT THỰC TẾ (REAL-TIME IMPLEMENTATION) ---
// Đây là "nhân viên" thực thi bản hợp đồng trên bằng cách lấy data từ OS
pub struct RealTimeProvider {
    sys: System,
}

impl Default for RealTimeProvider {
    fn default() -> Self {
        let mut sys = System::new_all();
        // Khởi tạo dữ liệu lần đầu để tránh các giá trị 0 ban đầu
        sys.refresh_all();
        Self { sys }
    }
}

impl RealTimeProvider {
    // Khởi tạo một RealTimeProvider mới.
    // Thực tế gọi đến implementation của Default.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl DataProvider for RealTimeProvider {
    async fn fetch_data(&mut self) -> SystemPulse<'_> {
        // Refresh các thành phần cụ thể
        self.sys.refresh_cpu();      // Cập nhật % CPU
        self.sys.refresh_memory();   // Cập nhật RAM
        self.sys.refresh_processes();// Cập nhật danh sách tiến trình

        let processes = self.sys.processes().values().map(|p| {
            ProcessPulse {
                pid: p.pid().as_u32(),
                // Zero-copy: Mượn tên trực tiếp từ buffer nội bộ của sysinfo
                name: Cow::Borrowed(p.name()),
                cpu_usage: p.cpu_usage(),
                mem_usage: p.memory(),
            }
        }).collect();

        SystemPulse {
            // Lấy CPU usage tổng quát của toàn hệ thống
            total_cpu: self.sys.global_cpu_info().cpu_usage(),
            total_mem: self.sys.total_memory(),
            free_mem: self.sys.free_memory(),
            processes,
        }
    }
}

// --- PHẦN 3: UNIT-TEST IMPLEMENTATION ---
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_realtime_provider() {
        let mut provider = RealTimeProvider::new();
        let data = provider.fetch_data().await;

        assert!(data.total_mem > 0);
        println!(" *** Total CPU: {}%", data.total_cpu);
        // Test xem có lấy được process nào không
        assert!(!data.processes.is_empty());
    }
}
