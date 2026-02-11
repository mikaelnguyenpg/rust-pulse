use crate::models::{SystemPulse, ProcessPulse};
use async_trait::async_trait;
use std::borrow::Cow;
use sysinfo::System;

// --- PHẦN 1: ĐỊNH NGHĨA GIAO DIỆN (TRAIT) ---
// Giống như một bản hợp đồng: "Bất cứ ai là DataProvider thì phải có hàm fetch_data"
#[async_trait]
pub trait DataProvider {
    // Trả về SystemPulse với Lifetime gắn liền với chính Provider
    async fn fetch_data(&mut self) -> SystemPulse<'_>;
}

// --- PHẦN 2: CÀI ĐẶT THỰC TẾ (REAL-TIME IMPLEMENTATION) ---
// Đây là "nhân viên" thực thi bản hợp đồng trên bằng cách lấy data từ OS
pub struct RealTimeProvider {
    sys: System,
}

impl RealTimeProvider {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }
}

#[async_trait]
impl DataProvider for RealTimeProvider {
    async fn fetch_data(&mut self) -> SystemPulse<'_> {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();

        let processes = self.sys.processes().values().map(|p| {
            ProcessPulse {
                pid: p.pid().as_u32(),
                name: Cow::Borrowed(p.name()),
                cpu_usage: p.cpu_usage(),
                mem_usage: p.memory(),
            }
        }).collect();

        SystemPulse {
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
        println!("Total CPU: {}%", data.total_cpu);
        // Test xem có lấy được process nào không
        assert!(!data.processes.is_empty());
    }
}
