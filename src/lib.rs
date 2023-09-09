#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fingerprinting {
    pub mod algorithm;
    pub mod communication;
    mod hanning;
    pub mod signature_format;
    pub mod user_agent;
}

mod utils {
    pub mod ffmpeg_wrapper;
}

use crate::fingerprinting::algorithm::SignatureGenerator;
// use std::error::Error;

pub fn from_buffer(buffer: Vec<f32>, sample_rate: u32) -> Result<serde_json::Value> {
    let sig = SignatureGenerator::make_signature_from_f32_buffer(buffer, sample_rate);
    let trck = fingerprinting::communication::recognize_song_from_signature(&sig)?;
    Ok(trck)
}

pub fn from_file(path: &str) -> Result<serde_json::Value> {
    let sig = SignatureGenerator::make_signature_from_file(path)?;
    let trck = fingerprinting::communication::recognize_song_from_signature(&sig)?;
    Ok(trck)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdError(#[from] Box<dyn std::error::Error>),
}
