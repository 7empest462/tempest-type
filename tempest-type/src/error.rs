use thiserror::Error;

#[derive(Error, Debug)]
pub enum TempestError {
    #[error("Audio Error: {0}")]
    AudioError(String),
    
    #[error("Transcription Error: {0}")]
    TranscriptionError(String),
    
    #[error("Ollama Error: {0}")]
    OllamaError(String),
    
    #[error("Configuration Error: {0}")]
    ConfigError(String),
    
    #[error("System Error: {0}")]
    SystemError(String),
}
