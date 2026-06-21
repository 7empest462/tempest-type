// Copyright (c) 2026 Robert Simens. All Rights Reserved.
// Licensed under the Tempest Type Source-Available License.
// See the LICENSE file in the repository root for full details.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use transcribe_rs::whisper_cpp::WhisperEngine;
use transcribe_rs::{SpeechModel, TranscribeOptions};

use crate::error::TempestError;

pub struct Transcriber {
    engine: WhisperEngine,
}

impl Transcriber {
    pub async fn new() -> Result<Self, TempestError> {
        let model_path = Self::ensure_model_downloaded().await?;

        let engine = WhisperEngine::load(&model_path)
            .map_err(|e| TempestError::TranscriptionError(e.to_string()))?;

        Ok(Self { engine })
    }

    pub fn transcribe(&mut self, audio_data: &[f32]) -> Result<String, TempestError> {
        // 1. ADD PADDING (Prevent cut-off)
        // Whisper often needs a generous tail of silence to avoid truncating the last word.
        let mut padded_audio = audio_data.to_vec();
        padded_audio.extend(vec![0.0; 32000]); // 2.0 seconds of silence padding at 16kHz

        let opts = TranscribeOptions {
            language: Some("en".to_string()),
            ..Default::default()
        };

        let result = self
            .engine
            .transcribe(&padded_audio, &opts)
            .map_err(|e| TempestError::TranscriptionError(e.to_string()))?;

        Ok(result.text.trim().to_string())
    }

    async fn ensure_model_downloaded() -> Result<PathBuf, TempestError> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let model_dir = Path::new(&home).join(".tempest-type").join("models");
        fs::create_dir_all(&model_dir).map_err(|e| TempestError::SystemError(e.to_string()))?;

        // ggml-small.en.bin
        let model_path = model_dir.join("ggml-small.en.bin");

        if !model_path.exists() {
            println!(
                "Downloading Whisper model (ggml-small.en.bin), ~480MB. This may take a minute..."
            );
            let url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.en.bin";
            let response = reqwest::get(url).await.map_err(|e| TempestError::SystemError(e.to_string()))?;
            let mut file = fs::File::create(&model_path).map_err(|e| TempestError::SystemError(e.to_string()))?;
            let bytes = response.bytes().await.map_err(|e| TempestError::SystemError(e.to_string()))?;
            file.write_all(&bytes).map_err(|e| TempestError::SystemError(e.to_string()))?;
            println!("Download complete!");
        }

        Ok(model_path)
    }
}
