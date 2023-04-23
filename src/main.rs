#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::Pos2;
use login_app_rs::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Login App",
        get_native_options(),
        Box::new(|cc| Box::new(LoginApp::new(cc))),
    )
}

fn get_native_options() -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_pos = Some(Pos2::new(250.0, 200.0));
    native_options.resizable = false;
    return native_options;
}
