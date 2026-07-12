use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::Emitter;
use tauri::{AppHandle, Manager};
use tauri_plugin_blec::models::{BleDevice, ScanFilter};
use tokio::sync::mpsc;
use uuid::{uuid, Uuid};

/// How long (seconds) a tag stays "present" after last seen
const SEEN_WINDOW_SECS: u64 = 60;
/// How long each BLE scan pass runs
const SCAN_SECS: u64 = 10;
/// Pause between scan passes
const LOOP_INTERVAL_SECS: u64 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagState {
    pub name: String,
    pub present: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindHubState {
    pub key: TagState,
    pub wallet: TagState,
}

fn frame_type_byte(data: &[u8]) -> Option<u8> {
    data.first().copied()
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

fn make_tag_state(name: &str, last_seen: u64) -> TagState {
    let now = now_secs();
    let present = last_seen > 0 && (now - last_seen) <= SEEN_WINDOW_SECS;
    TagState {
        name: name.to_string(),
        present,
        last_seen,
    }
}


/// Shared last-seen timestamps (key, wallet)
pub struct SeenTimes {
    key: u64,
    wallet: u64,
}


/// Tauri command: frontend can poll current state on demand
#[tauri::command]
pub fn get_state(state: tauri::State<Mutex<SeenTimes>>) -> FindHubState {
    let s = state.lock().unwrap();

    FindHubState {
        key: make_tag_state("Keys", s.key),
        wallet: make_tag_state("Wallet", s.wallet),
    }
}

async fn scan_results_loop(mut rx: mpsc::Receiver<Vec<BleDevice>>, app: AppHandle) {
    const FMDN: Uuid = uuid!("0000feaa-0000-1000-8000-00805f9b34fb");

    while let Some(devices) = rx.recv().await {
        for device in devices {
            for (uuid, data) in &device.service_data {
                if *uuid == FMDN {
                    if let Some(t) = frame_type_byte(data) {
                        match t {
                            0x41 => {
                                // key
                                let s = app.state::<Mutex<SeenTimes>>();
                                let mut s = s.lock().unwrap();
                                s.key = now_secs();

                                log::info!(
                                    "FindHub: key detected addr={} type=0x{:02x} eid={}",
                                    device.address,
                                    t,
                                    hex::encode(&data[1..data.len().min(21)])
                                );
                            }
                            0x40 => {
                                let s = app.state::<Mutex<SeenTimes>>();
                                let mut s = s.lock().unwrap();
                                s.wallet = now_secs();

                                log::info!(
                                    "FindHub: wallet detected addr={} type=0x{:02x} eid={}",
                                    device.address,
                                    t,
                                    hex::encode(&data[1..data.len().min(21)])
                                );
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
    }
}


pub async fn scanner_loop(app: AppHandle) {
    app.manage(Mutex::new(SeenTimes { key: 0, wallet: 0 }));
    log::info!("Test");

    let (scan_result_send, scan_result_recive) = mpsc::channel(1);
    let _ = tauri::async_runtime::spawn(scan_results_loop(scan_result_recive, app.clone()));

    let handler = tauri_plugin_blec::get_handler().unwrap();
    loop {
        log::info!("FindHub: starting {SCAN_SECS}s BLE scan");

        let res = handler
            .discover(
                Some(scan_result_send.clone()),
                SCAN_SECS * 1000,
                ScanFilter::None,
                true,
            )
            .await;


        if let Err(e) = res {
            log::error!("FindHub: scan start failed: {e}");
            continue;
        }
        tokio::time::sleep(Duration::from_secs(SCAN_SECS)).await;

        let res = handler.stop_scan().await;
        if let Err(e) = res {
            log::error!("FindHub: scan stop failed: {e}");
            continue;
        }

        log::info!("FindHub: scan complete");
        let state_snapshot = {
            let s = app.state::<Mutex<SeenTimes>>();
            let s = s.lock().unwrap();
            FindHubState {
                key: make_tag_state("Keys", s.key),
                wallet: make_tag_state("Wallet", s.wallet),
            }
        };

        let _ = app.emit("findhub://state", &state_snapshot);
        log::info!(
            "FindHub: emitted state key={} wallet={}",
            state_snapshot.key.present,
            state_snapshot.wallet.present
        );

        tokio::time::sleep(Duration::from_secs(LOOP_INTERVAL_SECS)).await;
    }
} 
