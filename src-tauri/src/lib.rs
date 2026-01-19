mod converter;
mod parser;

use converter::get_converter;
use parser::parse_ndjson;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConvertResult {
    pub zip_path: String,
    pub file_count: usize,
    pub image_count: usize,
}

#[tauri::command]
async fn convert_ndjson(
    file_path: String,
    format: String,
    output_path: String,
) -> Result<ConvertResult, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let data = parse_ndjson(&content).map_err(|e| format!("Failed to parse NDJSON: {}", e))?;

    let converter = get_converter(&format)
        .ok_or_else(|| format!("Unknown format: {}", format))?;

    let files = converter.convert(&data, &std::collections::HashMap::new());

    Ok(ConvertResult {
        zip_path: output_path,
        file_count: files.len(),
        image_count: 0,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_ndjson])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
