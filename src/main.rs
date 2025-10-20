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

    app.on_open_file(move || {
        let app = handle.unwrap();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Music Files", &["music"])
            .pick_file()
        {
            let path_str = path.display().to_string();
            app.set_file_path(SharedString::from(path_str.clone()));
            println!("Selected file: {:?}", path);

            // read file
            let json = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    return;
                }
            };

            // deserialize
            let song: Song = match serde_json::from_str(&json) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("JSON error: {}", e);
                    return;
                }
            };

            // create model and compute min width
            let measures_model = song_to_measures_model(&song);
            let min_width = compute_min_chord_width(&song);
            println!("Computed min chord width: {}", min_width);
            // update MainWindow properties — these are bound to SongView
            app.set_song_title(song.title.clone().into());
            app.set_artist(
                song.artist
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
                    .into(),
            );
            app.set_measures(measures_model.clone());
            app.set_min_chord_width(min_width);
        }
    });

    app.run()?;
    Ok(())
}

