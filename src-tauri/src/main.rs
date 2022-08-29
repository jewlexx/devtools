#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn base64_parse(input: &str, encode: bool) -> String {
    if encode {
        base64::encode(input)
    } else {
        String::from_utf8(base64::decode(input).unwrap()).unwrap()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![base64_parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
