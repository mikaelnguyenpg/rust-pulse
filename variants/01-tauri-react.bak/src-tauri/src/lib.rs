use core_engine::PulseManager;
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // 1. Khởi tạo PulseManager
      let manager = PulseManager::new(10);
      let mut receiver = manager.subscribe();

      // 2. Chạy vòng lặp lấy data (Pulse)
      // manager.start_loop(1000);
      let manager_clone = manager;
      tokio::spawn(async move {
        manager_clone.start_loop(1000).await;
      });

      // 3. Tạo một Task để lắng nghe từ manager và bắn lên UI
      let app_handle = app.handle().clone();
      tokio::spawn(async move {
        while let Ok(json_str) = receiver.recv().await {
          // Phát event 'pulse-update' kèm payload là chuỗi JSON
          let _ = app_handle.emit("pulse-update", json_str);
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
