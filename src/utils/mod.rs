use slint::SharedString;

use crate::models::Song;
use unicode_general_category::get_general_category;
pub fn lyric_to_word(lyric: &Option<String>) -> Vec<SharedString> {
    return lyric
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .map(SharedString::from)
        .collect();
}
fn approximate_visual_width(text: &str) -> f32 {
    text.chars()
        .map(|ch| {
            match get_general_category(ch) {
                Letter => 1.0,
                Mark => 0.5,
                _ if ch == '\u{094D}' => 0.2, // halant
                _ => 0.3,
            }
        })
        .sum()
}
pub fn compute_min_chord_width(song: &Song) -> i32 {
    let max_len = song
        .sections
        .iter()
        .flat_map(|section| section.measures.iter())
        .flat_map(|measure| measure.chords.iter())
        .map(|chord| {
            (approximate_visual_width(chord.lyric.clone().unwrap_or("".to_string()).as_str())
                / chord.beats as f32) as usize
        })
        .max()
        .unwrap_or(0);
    max_len as i32
}
