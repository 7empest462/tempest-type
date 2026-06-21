// Copyright (c) 2026 Robert Simens. All Rights Reserved.
// Licensed under the Tempest Type Source-Available License.
// See the LICENSE file in the repository root for full details.

use crate::AppEvent;
use rdev::{Event, EventType, listen};

pub fn start_listener(proxy: tao::event_loop::EventLoopProxy<AppEvent>, target_key: rdev::Key) {
    println!(
        "⌨️  Starting global hotkey listener (Target: {:?})...",
        target_key
    );
    std::thread::spawn(move || {
        let callback = move |event: Event| {
            let is_match = match event.event_type {
                EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                    if key == target_key {
                        true
                    } else if (target_key == rdev::Key::AltGr || target_key == rdev::Key::Alt) 
                           && (key == rdev::Key::AltGr || key == rdev::Key::Alt) {
                        // Mac keyboard quirk: Many keyboards send Left Option (Alt) even for the Right Option key
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };

            match event.event_type {
                EventType::KeyPress(_) if is_match => {
                    let _ = proxy.send_event(AppEvent::StartRecording(false));
                }
                EventType::KeyRelease(_) if is_match => {
                    let _ = proxy.send_event(AppEvent::StopRecording);
                }
                _ => {}
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!(
                "❌ Hotkey listener error: {:?}. This is usually a macOS Permission issue (Accessibility/Input Monitoring).",
                error
            );
        }
    });
}
