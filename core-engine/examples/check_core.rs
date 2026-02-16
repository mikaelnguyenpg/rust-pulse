use core_engine::{RealTimeProvider, DataProvider};

#[tokio::main]
async fn main() {
    let mut provider = RealTimeProvider::new();
    let data = provider.fetch_data().await;

    println!("--- RustPulse Core Engine check ---");

    // dbg!(&data); // Nó sẽ in ra cả file, dòng code và giá trị rất chi tiết.
    // println!("Data: {:#?}", data); // Dấu :#? sẽ xuống dòng và thụt lề giúp bạn dễ đọc object lớn.
    let mut display_data = data.clone();
    display_data.processes.truncate(5);
    println!("Data: {:#?}", display_data);
    // println!("Data: {}...", &format!("{:?}", data)[..200]);

    println!("Total Memory: {} KB ~ {} GB", data.total_mem, data.total_mem / 1024 / 1024);
    println!("Total CPU: {:.2}%", data.total_cpu);
    println!("Top 5 Processes:");

    let mut processes = data.processes;
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    for p in processes.iter().take(5) {
        println!("PID: {} | Name: {} | CPU: {:.2}%", p.pid, p.name, p.cpu_usage);
    }
}
