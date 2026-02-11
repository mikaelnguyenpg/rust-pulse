use serde::Serialize;
use std::borrow::Cow;

// Lifetime `'a`. Nó ràng buộc rằng dữ liệu trong Struct này phải còn sống chừng nào cái buffer của `sysinfo` còn sống.
#[derive(Serialize, Clone, Debug)]
pub struct ProcessPulse<'a> {
    pub pid: u32,
    // Cow (Copy-on-Write) giúp tránh clone string trừ khi thực sự cần sửa đổi
    pub name: Cow<'a, str>,
    pub cpu_usage: f32,
    pub mem_usage: u64,
}

#[derive(Serialize, Clone, Debug)]
pub struct SystemPulse<'a> {
    pub total_cpu: f32,
    pub total_mem: u64,
    pub free_mem: u64,
    pub processes: Vec<ProcessPulse<'a>>,
}
