use core_engine::PulseManager;
use tauri::Emitter;
use dotenvy::dotenv;
use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  dotenv().ok();

  let event_name = env::var("VITE_EVENT_PULSE_UPDATE")
    .unwrap_or_else(|_| "pulse_update".to_string());
    
  tauri::Builder::default()
    .setup(|app| {
      let manager = PulseManager::new(10);
      let mut receiver = manager.subscribe();

      tauri::async_runtime::spawn(async move {
        manager.start_loop(1000).await;
      });

      let app_handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        while let Ok(json_str) = receiver.recv().await {
          println!("Backend emitting pulse: {}", json_str);
          let _ = app_handle.emit(&event_name, json_str);
        }
      });
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
