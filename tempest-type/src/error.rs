use thiserror::Error;

#[derive(Error, Debug)]
pub enum TempestError {
    #[error("Audio Error: {0}")]
    Audio(String),

    #[error("Transcription Error: {0}")]
    Transcription(String),

    #[error("Ollama Error: {0}")]
    Ollama(String),

    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("System Error: {0}")]
    System(String),
}
