pub mod models;
pub mod monitor;
pub mod manager;

// Re-export để bên ngoài gọi cho gọn: core_engine::SystemPulse
pub use models::*;
pub use monitor::*;
pub use manager::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
