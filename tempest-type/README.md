# Tempest Type 🎙️

Tempest Type is an AI-powered, system-wide dictation and keyboard macro tool for macOS. It records your voice via a global hotkey, instantly transcribes it locally using `whisper.cpp` (ggml-small.en), cleans up transcription errors and stutters using a local Ollama model (like Qwen2.5 or DeepSeek), and automatically injects the perfectly punctuated text directly into whatever application you are currently typing in.

## Features
- **Local Privacy**: 100% offline transcription and text correction.
- **Global Hotkey**: Press and hold `Right Option` anywhere on your Mac to dictate.
- **Auto-Correction**: Eliminates the "um"s, "ah"s, and mic static hallucinations using strict AI refinement rules.
- **Smart Memo Mode**: Double-tap `Right Option` to toggle "Memo Mode" which will summarize your next recording instead of transcribing it literally.
- **System Tray Integration**: Simple, unobtrusive tray icon to show recording status and quick actions.
- **Voice Activity Detection**: Safely ignores silence and background static to prevent Whisper hallucinations.

---

## 🛠️ Installation & Building

Tempest Type is packaged as a macOS `.app` bundle to properly request and retain Microphone and Accessibility permissions, as macOS heavily restricts raw background binaries.

### Prerequisites
1. **Rust**: Ensure you have Rust and Cargo installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).
2. **Ollama**: You must have [Ollama](https://ollama.com/) installed and running locally.
3. **Model**: Pull the required model via Ollama (e.g., `ollama pull qwen2.5:14b-instruct`). You can change the model used in the configuration file later.

### Building the App
We use a build script to compile the binary and package it into a proper `.app` bundle with an `Info.plist` and code signature.

```bash
cd tempest-type
./build_mac_app.sh
```

This will generate `TempestType.app` in the project directory.

---

## 🚀 Running the App & Quirks (CRITICAL)

Because `TempestType.app` intercepts global keyboard events and accesses your microphone, **macOS will throw strict security restrictions at it**. 

### 1. First Launch
Launch the app by double-clicking `TempestType.app` in Finder, or running `open TempestType.app` in your terminal.
- macOS will likely prompt you for **Microphone** access. Click **Allow**.
- macOS will prompt you for **Accessibility** access. You must open **System Settings -> Privacy & Security -> Accessibility**, and enable `TempestType.app`.

### 2. The Recompilation Quirk (Permissions Reset)
**IMPORTANT:** Whenever you edit the code and re-run `./build_mac_app.sh`, macOS will silently invalidate your permissions because the app's code signature changed.

If you rebuild the app and it stops working:
- **Hotkey doesn't trigger?** Go to **Accessibility**, select `TempestType.app`, click the minus (`-`) button to delete it, then click the plus (`+`) button to re-add it.
- **Microphone records silence (or hallucinations)?** Go to **Microphone**, delete `TempestType.app` with the minus (`-`) button, and re-add it. macOS is notorious for silently feeding the app pure zeroes (silence) if the mic permission is subtly invalidated by a rebuild.

### 3. VAD and Silence
The app features Voice Activity Detection (VAD). If you tap the hotkey but don't speak, the app will drop the audio to prevent Whisper from hallucinating random tutorial strings (like "Xcode Command Line Tools"). You will see a `Skipping` log if you run it from the terminal.

---

## ⚙️ Configuration

The first time you run the app, it generates a configuration file. You can find it at:
`~/.config/tempest-type/default-config.toml`

**Default config:**
```toml
model = "qwen2.5:3b"
hotkey = "AltGr"
```

You can change the `model` to whichever Ollama model you prefer, or alter the `hotkey` if you want a different key (e.g. `F4`).

---

## 📜 License

This software is provided under the **Tempest Type Source-Available License**. 
See the `LICENSE` file for details. Commercial use, reselling, or SaaS distribution is strictly prohibited without explicit written permission.
