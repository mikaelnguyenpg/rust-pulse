use core_engine::{RealTimeProvider, DataProvider};

#[tokio::main]
async fn main() {
    let mut provider = RealTimeProvider::new();
    let data = provider.fetch_data().await;

    println!("--- RustPulse Core Engine check ---");
    println!("Total Memory: {} KB", data.total_mem);
    println!("Total CPU: {:.2}%", data.total_cpu);
    println!("Top 5 Processes:");

    let mut processes = data.processes;
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    for p in processes.iter().take(5) {
        println!("PID: {} | Name: {} | CPU: {:.2}%", p.pid, p.name, p.cpu_usage);
    }
}
