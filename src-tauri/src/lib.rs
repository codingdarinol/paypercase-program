mod commands;
mod db;
mod models;
mod state;

use db::local;
use state::AppState;

fn get_db_path() -> std::path::PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        let app_dir = data_dir.join("PayPerCase");
        std::fs::create_dir_all(&app_dir).ok();
        app_dir.join("paypercase.db")
    } else {
        std::path::PathBuf::from("paypercase.db")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = get_db_path();
    let conn = rusqlite::Connection::open(&db_path)
        .expect("Tidak dapat membuka database lokal paypercase.db");

    local::init_config_db(&conn).expect("Tidak dapat menginisialisasi database lokal");

    let state = AppState::new(conn);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_all_pttypes,
            commands::settings::save_pttype,
            commands::settings::delete_pttype,
            commands::settings::get_all_procedures,
            commands::settings::save_procedure,
            commands::settings::delete_procedure,
            commands::settings::get_all_drugs,
            commands::settings::save_drug,
            commands::settings::delete_drug,
            commands::settings::get_all_providers,
            commands::settings::save_provider,
            commands::settings::delete_provider,
            commands::settings::get_payout_options,
            commands::settings::add_payout_option,
            commands::settings::delete_payout_option,
            commands::pending::upsert_pending,
            commands::pending::get_pending_export,
            commands::pending::delete_pending_by_id,
            commands::pending::export_pending_csv,
            commands::delete::preview_delete_range,
            commands::delete::delete_pending_range,
            commands::delete::get_monthly_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PayPerCase application");
}
