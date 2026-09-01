// Prevents an additional console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod detection;
mod fim;
mod models;
mod network;
mod state;
mod storage;

use state::AppState;
use std::time::Duration;
use storage::{Database, Repository};
use tauri::Manager;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");

            let db = Database::new(data_dir).expect("failed to initialize database");
            let state = AppState::new(db);

            let db_for_metrics = state.db.clone();
            let engine_for_flows = state.engine.clone();
            let db_for_flows = state.db.clone();

            app.manage(state);

            // Background task: sample system performance metrics every 2s.
            tauri::async_runtime::spawn(async move {
                let mut sys = sysinfo::System::new_all();
                loop {
                    sys.refresh_cpu_usage();
                    sys.refresh_memory();
                    let cpu = sys.global_cpu_usage();
                    let ram_used = sys.used_memory() / 1024 / 1024;
                    let ram_total = sys.total_memory() / 1024 / 1024;

                    let networks = sysinfo::Networks::new_with_refreshed_list();
                    let mut rx = 0u64;
                    let mut tx = 0u64;
                    for (_iface, data) in &networks {
                        rx += data.total_received();
                        tx += data.total_transmitted();
                    }

                    let storage_used_mb = 0u64; // populated by periodic DB file size check

                    let metrics = models::SystemMetrics {
                        timestamp: chrono::Utc::now(),
                        cpu_percent: cpu,
                        ram_used_mb: ram_used,
                        ram_total_mb: ram_total,
                        network_rx_bytes: rx,
                        network_tx_bytes: tx,
                        events_per_sec: 0.0,
                        detection_latency_ms: 0.0,
                        storage_used_mb,
                    };

                    let repo = Repository::new(&db_for_metrics);
                    let _ = repo.insert_metrics(&metrics);

                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });

            // Background task: sample active connections every 5s and run
            // them through the detection engine.
            tauri::async_runtime::spawn(async move {
                loop {
                    let flows = network::sample_flows();
                    if !flows.is_empty() {
                        let _ = engine_for_flows.process_flows(&db_for_flows, &flows);
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::devices::scan_network,
            commands::devices::list_devices,
            commands::devices::scan_device_ports,
            commands::devices::get_topology,
            commands::events::list_events,
            commands::events::search_events,
            commands::alerts::list_alerts,
            commands::alerts::acknowledge_alert,
            commands::incidents::list_incidents,
            commands::incidents::create_incident_from_alert,
            commands::incidents::update_incident_status,
            commands::incidents::add_incident_note,
            commands::rules::list_rules,
            commands::rules::create_rule,
            commands::rules::toggle_rule,
            commands::rules::delete_rule,
            commands::ai_commands::ai_correlate_event,
            commands::ai_commands::ai_explain_alert,
            commands::ai_commands::ai_posture_summary,
            commands::fim_commands::add_watched_file,
            commands::fim_commands::list_watched_files,
            commands::fim_commands::remove_watched_file,
            commands::fim_commands::run_integrity_scan,
            commands::vault_commands::vault_setup,
            commands::vault_commands::vault_unlock,
            commands::vault_commands::vault_lock,
            commands::vault_commands::vault_put,
            commands::vault_commands::vault_get,
            commands::vault_commands::vault_delete,
            commands::vault_commands::vault_list_keys,
            commands::metrics::get_recent_metrics,
            commands::flows_commands::list_flows,
            commands::flows_commands::refresh_flows,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AEGIS");
}
