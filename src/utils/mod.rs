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
const CHORDS: &[&str] = &["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const ENHARMONIC: &[(&str, &str)] = &[
    ("C#", "Db"),
    ("D#", "Eb"),
    ("F#", "Gb"),
    ("G#", "Ab"),
    ("A#", "Bb"),
];

fn normalize_root(root: &str) -> &str {
    for &(sharp, flat) in ENHARMONIC {
        if root == flat {
            return sharp;
        }
    }
    root
}

fn split_chord(chord: &str) -> (&str, &str) {
    // Root is either 1 or 2 chars (C, D#, Eb, etc.)
    let chars: Vec<char> = chord.chars().collect();
    if chars.len() >= 2 && (chars[1] == '#' || chars[1] == 'b') {
        (&chord[..2], &chord[2..])
    } else {
        (&chord[..1], &chord[1..])
    }
}

pub fn get_transposed_chord(chord: &str, pos: isize) -> String {
    let (root, suffix) = split_chord(chord);
    let root_normalized = normalize_root(root);

    let index = CHORDS.iter().position(|&c| c == root_normalized)
        .expect("Chord root not found") as isize;
    
    // Wrap around properly for negative positions
    let transposed_index = ((index - pos) % CHORDS.len() as isize + CHORDS.len() as isize) % CHORDS.len() as isize;
    let transposed_root = CHORDS[transposed_index as usize];

    format!("{}{}", transposed_root, suffix)
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
