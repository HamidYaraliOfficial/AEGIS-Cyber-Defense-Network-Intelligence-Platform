use crate::models::Device;
use crate::network::{self, topology::Topology};
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn scan_network(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    {
        let mut in_progress = state.scan_in_progress.write().await;
        if *in_progress {
            return Err("A scan is already in progress".into());
        }
        *in_progress = true;
    }

    let gateway = network::discovery::detect_gateway();
    let result = network::discover_devices(gateway).await;

    *state.scan_in_progress.write().await = false;

    match result {
        Ok(devices) => {
            let repo = Repository::new(&state.db);
            for d in &devices {
                repo.upsert_device(d).map_err(|e| e.to_string())?;
            }
            Ok(devices)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn list_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    let repo = Repository::new(&state.db);
    repo.list_devices().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn scan_device_ports(
    state: State<'_, AppState>,
    device_ip: String,
    deep: bool,
) -> Result<Vec<u16>, String> {
    let ports = network::scan_ports(device_ip.clone(), deep).await;

    let repo = Repository::new(&state.db);
    if let Ok(Some(mut device)) = repo.find_device_by_ip(&device_ip) {
        device.open_ports = ports.clone();
        device.risk_score = compute_risk_score(&ports);
        let _ = repo.upsert_device(&device);
    }

    Ok(ports)
}

#[tauri::command]
pub async fn get_topology(state: State<'_, AppState>) -> Result<Topology, String> {
    let repo = Repository::new(&state.db);
    let devices = repo.list_devices().map_err(|e| e.to_string())?;
    Ok(network::build_topology(&devices))
}

fn compute_risk_score(open_ports: &[u16]) -> u8 {
    let risky: &[u16] = &[21, 23, 135, 139, 445, 3389, 5900, 1433, 3306, 5432, 27017, 6379];
    let mut score: u32 = 0;
    for p in open_ports {
        if risky.contains(p) {
            score += 15;
        } else {
            score += 3;
        }
    }
    score.min(100) as u8
}
