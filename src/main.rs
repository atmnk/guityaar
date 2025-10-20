#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{SharedString};
use std::error::Error;
use std::fs;
pub mod models;
pub mod utils;
use crate::models::*;
use crate::utils::*;
fn main() -> Result<(), Box<dyn Error>> {
    let app = MainWindow::new()?;
    let handle = app.as_weak();
    app.on_request_increase_value({
        let ui_handle = app.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_capo(ui.get_capo() + 1);
            let path=ui.get_file_path().to_string();
            reset_app_path(ui, path);
        }
    });
    app.on_request_decrease_value({
        let ui_handle = app.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_capo(ui.get_capo() - 1);
            let path=ui.get_file_path().to_string();
            reset_app_path(ui, path);
        }
    });
    app.on_open_file(move || {
        let app = handle.unwrap();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Music Files", &["music"])
            .pick_file()
        {
            reset_app_path(app, path.display().to_string())
        }
    });

    app.run()?;
    Ok(())
}

fn reset_app_path(app: MainWindow, path_str: String) {
    app.set_file_path(SharedString::from(path_str.clone()));
    let json = match fs::read_to_string(path_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Read error: {}", e);
            return
        }
    };
    let song: Song = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("JSON error: {}", e);
            return
        }
    };
    let measures_model = song_to_measures_model(&song,app.get_capo());
    let min_width = compute_min_chord_width(&song);
    println!("Computed min chord width: {}", min_width);
    app.set_song_title(song.title.clone().into());
    app.set_artist(
        song.artist
            .clone()
            .unwrap_or_else(|| "Unknown".to_string())
            .into(),
    );
    app.set_measures(measures_model.clone());
    app.set_min_chord_width(min_width);

    // read file

    // deserialize

    // create model and compute min width
    // update MainWindow properties — these are bound to SongView
    return
}

