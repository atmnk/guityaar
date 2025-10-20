use serde::{Deserialize, Serialize};
use crate::utils::{get_transposed_chord, lyric_to_word};
use slint::{ModelRc, VecModel};
use std::rc::Rc;
slint::include_modules!();
#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub artist: Option<String>,
    pub tempo: u32,
    pub key: Option<String>,
    pub time_signatures: Vec<TimeSignature>,
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSignature {
    pub start_measure: u32,
    pub beats_per_measure: u8,
    pub beat_unit: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub name: String,
    pub measures: Vec<Measure>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measure {
    pub number: u32,
    pub chords: Vec<Chord>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chord {
    pub beats: i32,
    pub name: String,
    pub lyric: Option<String>,
}

fn chord_to_ui_chord(chord: &Chord,capo:i32) -> UIChord {
    let words = lyric_to_word(&chord.lyric);

    UIChord {
        name: get_transposed_chord(&chord.name, capo as isize).clone().into(),

        lyric_words: ModelRc::from(Rc::new(VecModel::from(words.clone()))),
        beats: chord.beats,
        lyric_word_count: words.len() as i32,
    }
}
fn measure_to_ui_measure(measure: &Measure,capo:i32) -> UIMeasure {
    let chords_vec: Vec<UIChord> = measure.chords.iter().map(|ch|chord_to_ui_chord(ch,capo)).collect();
    UIMeasure {
        chords: ModelRc::from(Rc::new(VecModel::from(chords_vec))),
    }
}

pub fn song_to_measures_model(song: &Song, capo : i32) -> ModelRc<UIMeasure> {
    let measures_vec: Vec<UIMeasure> = song
        .sections
        .iter()
        .flat_map(|section| section.measures.iter().map(|m|measure_to_ui_measure(m,capo)))
        .collect();
    ModelRc::new(VecModel::from(measures_vec))
}