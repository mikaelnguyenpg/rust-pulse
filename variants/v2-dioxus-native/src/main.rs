use dioxus::prelude::*;
use core_engine::{PulseManager, SystemPulse};

fn main() {
    // Khởi tạo Dioxus Desktop
    println!("Hello, world!");
    launch(app);
}

fn app() -> Element {
    // 1. Khởi tạo State để lưu trữ Pulse
    let mut pulse_state = use_signal(|| None::<SystemPulse<'static>>);

    // 2. Chạy một Coroutine (Task ngầm) để nhận dữ liệu từ Core
    use_future(move || async move {
        // Khởi tạo manager bên trong async task
        let manager = PulseManager::new(10);
        let mut receiver = manager.subscribe();

        // Chạy vòng lặp lấy dữ liệu (1Hz)
        manager.start_loop(1000).await;

        while let Ok(data) = receiver.recv().await {
            pulse_state.set(Some(data));
        }
    });

    // 3. Render UI
    let p = pulse_state.read();

    rsx! {
        style { {include_str!("../assets/main.css")} }
        div {
            class: "container",
            h1 { "Rust Pulse Native (Dioxus)" }

            if let Some(pulse) = p.as_ref() {
                div {
                    div { class: "summary",
                        div { class: "stat-card",
                            label { "System CPU" }
                            div { class: "value", "{pulse.total_cpu:.1}%" }
                        }
                        div { class: "stat-card",
                            label { "Memory Usage" }
                            div { class: "value", "{(pulse.total_mem - pulse.free_mem) / 1024 / 1024 / 1024:.2} GB" }
                        }
                    }
                    table { class: "table-wrapper",
                        thead {
                            tr {
                                th { "PID" }
                                th { "Proceess Name" }
                                th { class: "right", "CPU (%)" }
                                th { class: "right", "Mem (MB)" }
                            }
                        }
                        tbody {
                            for proc in &pulse.processes {
                                tr {
                                    key: "{proc.pid}",
                                    td { "{proc.pid}" }
                                    td { class: "name", "{proc.name}" }
                                    td { class: "right cpu-val", "{proc.cpu_usage:.2}" }
                                    td { class: "right", "{proc.mem_usage / 1024 / 1024}"}
                                }
                            }
                        }
                    }
                }
            } else {
                p { "Scanning system..." }
            }
        }
    }
}
