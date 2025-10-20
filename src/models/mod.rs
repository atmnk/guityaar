use serde::{Deserialize, Serialize};
use crate::utils::lyric_to_word;
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

fn chord_to_ui_chord(chord: &Chord) -> UIChord {
    let words = lyric_to_word(&chord.lyric);

    UIChord {
        name: chord.name.clone().into(),

        lyric_words: ModelRc::from(Rc::new(VecModel::from(words.clone()))),
        beats: chord.beats,
        lyric_word_count: words.len() as i32,
    }
}
fn measure_to_ui_measure(measure: &Measure) -> UIMeasure {
    let chords_vec: Vec<UIChord> = measure.chords.iter().map(chord_to_ui_chord).collect();
    UIMeasure {
        chords: ModelRc::from(Rc::new(VecModel::from(chords_vec))),
    }
}

pub fn song_to_measures_model(song: &Song) -> ModelRc<UIMeasure> {
    let measures_vec: Vec<UIMeasure> = song
        .sections
        .iter()
        .flat_map(|section| section.measures.iter().map(measure_to_ui_measure))
        .collect();
    ModelRc::new(VecModel::from(measures_vec))
}