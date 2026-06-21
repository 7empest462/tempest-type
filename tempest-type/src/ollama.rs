// Copyright (c) 2026 Robert Simens. All Rights Reserved.
// Licensed under the Tempest Type Source-Available License.
// See the LICENSE file in the repository root for full details.

use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

use crate::error::TempestError;

pub async fn cleanup_text(raw_text: &str, model: &str) -> Result<String, TempestError> {
    let system_msg = "You are a world-class professional editor and transcription specialist.\n\n\
                      TASKS:\n\
                      1. REWRITE the raw speech-to-text input into professional, clear, and perfectly punctuated English.\n\
                      2. FIX transcription hallucinations: use context to infer intended words.\n\
                      3. STRIP all non-speech artifacts: remove parenthetical sound descriptions like '(Air whooshing)', '(beep)', '(burp)', '[SOUND]', or '(upbeat music)'.\n\
                      4. REMOVE all stutters, repetitions, and filler words (um, ah, like).\n\
                      5. POLISH for professional flow: ensure sentences connect logically and elegantly.\n\
                      6. CRITICAL: DO NOT alter the core meaning, vocabulary, or stylistic choices unnecessarily. Only fix grammar and stutters.\n\
                      7. Return ONLY the refined, final text. NEVER include conversational filler, pleasantries, or introductory phrases like 'Here is the refined text:'. If the input is only noise or artifacts, return an empty string.";

    let user_msg = format!(
        "Please professionally refine this raw transcription: {}",
        raw_text
    );

    let ollama = Ollama::default();

    let messages = vec![
        ChatMessage::system(system_msg.to_string()),
        ChatMessage::user(user_msg),
    ];

    let options = ModelOptions::default().temperature(0.1).num_predict(500);

    let request = ChatMessageRequest::new(model.to_string(), messages).options(options);

    let res = ollama
        .send_chat_messages(request)
        .await
        .map_err(|e| TempestError::OllamaError(e.to_string()))?;

    Ok(res.message.content.trim().to_string())
}

pub async fn summarize_memo(raw_text: &str, model: &str) -> Result<String, TempestError> {
    let system_msg = "You are a professional executive assistant. Your task is to summarize meeting transcripts into a structured, highly readable format.\n\n\
                      FORMAT REQUIREMENTS:\n\
                      - **Executive Summary**: A brief 1-3 sentence overview of the meeting's purpose and outcome.\n\
                      - **Key Discussion Points**: A bulleted list of the main topics discussed.\n\
                      - **Action Items**: A clear, actionable list of tasks and next steps (include assignees if mentioned).\n\n\
                      RULES:\n\
                      1. Output ONLY the formatted summary. Do not include introductory conversational filler like 'Here is the summary:'.\n\
                      2. Do not use 'Thinking' blocks or output your internal reasoning.";

    let user_msg = format!("Summarize this: {}", raw_text);

    let ollama = Ollama::default();

    let messages = vec![
        ChatMessage::system(system_msg.to_string()),
        ChatMessage::user(user_msg),
    ];

    let options = ModelOptions::default().temperature(0.2).num_predict(1000);

    let request = ChatMessageRequest::new(model.to_string(), messages).options(options);

    let res = ollama
        .send_chat_messages(request)
        .await
        .map_err(|e| TempestError::OllamaError(e.to_string()))?;

    Ok(res.message.content.trim().to_string())
}
