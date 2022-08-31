#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

enum ColorModeTypes {
    Dark,
    Light,
}

impl From<dark_light::Mode> for ColorModeTypes {
    fn from(mode: dark_light::Mode) -> Self {
        match mode {
            dark_light::Mode::Dark => ColorModeTypes::Dark,
            dark_light::Mode::Light => ColorModeTypes::Light,
        }
    }
}

struct ColorMode(pub ColorModeTypes);

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
        .manage(ColorMode(dark_light::detect().into()))
        .invoke_handler(tauri::generate_handler![base64_parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
