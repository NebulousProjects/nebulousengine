use bevy::prelude::*;
use serde::*;

pub fn serialize<S>(keycode: &KeyCode, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(keycode.type_name())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyCode, D::Error> where D: Deserializer<'de> {
    let value = String::deserialize(deserializer)?;
    let value = value.as_str();
    match value {
        "Key1" => Ok(KeyCode::Key1),
        "Key2" => Ok(KeyCode::Key2),
        "Key3" => Ok(KeyCode::Key3),
        "Key4" => Ok(KeyCode::Key4),
        "Key5" => Ok(KeyCode::Key5),
        "Key6" => Ok(KeyCode::Key6),
        "Key7" => Ok(KeyCode::Key7),
        "Key8" => Ok(KeyCode::Key8),
        "Key9" => Ok(KeyCode::Key9),
        "Key0" => Ok(KeyCode::Key0),
        "A" => Ok(KeyCode::A),
        "B" => Ok(KeyCode::B),
        "C" => Ok(KeyCode::C),
        "D" => Ok(KeyCode::D),
        "E" => Ok(KeyCode::E),
        "F" => Ok(KeyCode::F),
        "G" => Ok(KeyCode::G),
        "H" => Ok(KeyCode::H),
        "I" => Ok(KeyCode::I),
        "J" => Ok(KeyCode::J),
        "K" => Ok(KeyCode::K),
        "L" => Ok(KeyCode::L),
        "M" => Ok(KeyCode::M),
        "N" => Ok(KeyCode::N),
        "O" => Ok(KeyCode::O),
        "P" => Ok(KeyCode::P),
        "Q" => Ok(KeyCode::Q),
        "R" => Ok(KeyCode::R),
        "S" => Ok(KeyCode::S),
        "T" => Ok(KeyCode::T),
        "U" => Ok(KeyCode::U),
        "V" => Ok(KeyCode::V),
        "W" => Ok(KeyCode::W),
        "X" => Ok(KeyCode::X),
        "Y" => Ok(KeyCode::Y),
        "Z" => Ok(KeyCode::Z),
        "Escape" => Ok(KeyCode::Escape),
        "F1" => Ok(KeyCode::F1),
        "F2" => Ok(KeyCode::F2),
        "F3" => Ok(KeyCode::F3),
        "F4" => Ok(KeyCode::F4),
        "F5" => Ok(KeyCode::F5),
        "F6" => Ok(KeyCode::F6),
        "F7" => Ok(KeyCode::F7),
        "F8" => Ok(KeyCode::F8),
        "F9" => Ok(KeyCode::F9),
        "F10" => Ok(KeyCode::F10),
        "F11" => Ok(KeyCode::F11),
        "F12" => Ok(KeyCode::F12),
        "F13" => Ok(KeyCode::F13),
        "F14" => Ok(KeyCode::F14),
        "F15" => Ok(KeyCode::F15),
        "F16" => Ok(KeyCode::F16),
        "F17" => Ok(KeyCode::F17),
        "F18" => Ok(KeyCode::F18),
        "F19" => Ok(KeyCode::F19),
        "F20" => Ok(KeyCode::F20),
        "F21" => Ok(KeyCode::F21),
        "F22" => Ok(KeyCode::F22),
        "F23" => Ok(KeyCode::F23),
        "F24" => Ok(KeyCode::F24),
        "Snapshot" => Ok(KeyCode::Snapshot),
        "Scroll" => Ok(KeyCode::Scroll),
        "Pause" => Ok(KeyCode::Pause),
        "Insert" => Ok(KeyCode::Insert),
        "Home" => Ok(KeyCode::Home),
        "Delete" => Ok(KeyCode::Delete),
        "End" => Ok(KeyCode::End),
        "PageDown" => Ok(KeyCode::PageDown),
        "PageUp" => Ok(KeyCode::PageUp),
        "Left" => Ok(KeyCode::Left),
        "Up" => Ok(KeyCode::Up),
        "Right" => Ok(KeyCode::Right),
        "Down" => Ok(KeyCode::Down),
        "Back" => Ok(KeyCode::Back),
        "Return" => Ok(KeyCode::Return),
        "Space" => Ok(KeyCode::Space),
        "Compose" => Ok(KeyCode::Compose),
        "Caret" => Ok(KeyCode::Caret),
        "Numlock" => Ok(KeyCode::Numlock),
        "Numpad0" => Ok(KeyCode::Numpad0),
        "Numpad1" => Ok(KeyCode::Numpad1),
        "Numpad2" => Ok(KeyCode::Numpad2),
        "Numpad3" => Ok(KeyCode::Numpad3),
        "Numpad4" => Ok(KeyCode::Numpad4),
        "Numpad5" => Ok(KeyCode::Numpad5),
        "Numpad6" => Ok(KeyCode::Numpad6),
        "Numpad7" => Ok(KeyCode::Numpad7),
        "Numpad8" => Ok(KeyCode::Numpad8),
        "Numpad9" => Ok(KeyCode::Numpad9),
        "AbntC1" => Ok(KeyCode::AbntC1),
        "AbntC2" => Ok(KeyCode::AbntC2),
        "NumpadAdd" => Ok(KeyCode::NumpadAdd),
        "Apostrophe" => Ok(KeyCode::Apostrophe),
        "Apps" => Ok(KeyCode::Apps),
        "Asterisk" => Ok(KeyCode::Asterisk),
        "Plus" => Ok(KeyCode::Plus),
        "At" => Ok(KeyCode::At),
        "Ax" => Ok(KeyCode::Ax),
        "Backslash" => Ok(KeyCode::Backslash),
        "Calculator" => Ok(KeyCode::Calculator),
        "Capital" => Ok(KeyCode::Capital),
        "Colon" => Ok(KeyCode::Colon),
        "Comma" => Ok(KeyCode::Comma),
        "Convert" => Ok(KeyCode::Convert),
        "NumpadDecimal" => Ok(KeyCode::NumpadDecimal),
        "NumpadDivide" => Ok(KeyCode::NumpadDivide),
        "Equals" => Ok(KeyCode::Equals),
        "Grave" => Ok(KeyCode::Grave),
        "Kana" => Ok(KeyCode::Kana),
        "Kanji" => Ok(KeyCode::Kanji),
        "AltLeft" => Ok(KeyCode::AltLeft),
        "BracketLeft" => Ok(KeyCode::BracketLeft),
        "ControlLeft" => Ok(KeyCode::ControlLeft),
        "ShiftLeft" => Ok(KeyCode::ShiftLeft),
        "SuperLeft" => Ok(KeyCode::SuperLeft),
        "Mail" => Ok(KeyCode::Mail),
        "MediaSelect" => Ok(KeyCode::MediaSelect),
        "MediaStop" => Ok(KeyCode::MediaStop),
        "Minus" => Ok(KeyCode::Minus),
        "NumpadMultiply" => Ok(KeyCode::NumpadMultiply),
        "Mute" => Ok(KeyCode::Mute),
        "MyComputer" => Ok(KeyCode::MyComputer),
        "NavigateForward" => Ok(KeyCode::NavigateForward),
        "NavigateBackward" => Ok(KeyCode::NavigateBackward),
        "NextTrack" => Ok(KeyCode::NextTrack),
        "NoConvert" => Ok(KeyCode::NoConvert),
        "NumpadComma" => Ok(KeyCode::NumpadComma),
        "NumpadEnter" => Ok(KeyCode::NumpadEnter),
        "NumpadEquals" => Ok(KeyCode::NumpadEquals),
        "Oem102" => Ok(KeyCode::Oem102),
        "Period" => Ok(KeyCode::Period),
        "PlayPause" => Ok(KeyCode::PlayPause),
        "Power" => Ok(KeyCode::Power),
        "PrevTrack" => Ok(KeyCode::PrevTrack),
        "AltRight" => Ok(KeyCode::AltRight),
        "BracketRight" => Ok(KeyCode::BracketRight),
        "ControlRight" => Ok(KeyCode::ControlRight),
        "ShiftRight" => Ok(KeyCode::ShiftRight),
        "SuperRight" => Ok(KeyCode::SuperRight),
        "Semicolon" => Ok(KeyCode::Semicolon),
        "Slash" => Ok(KeyCode::Slash),
        "Sleep" => Ok(KeyCode::Sleep),
        "Stop" => Ok(KeyCode::Stop),
        "NumpadSubtract" => Ok(KeyCode::NumpadSubtract),
        "Sysrq" => Ok(KeyCode::Sysrq),
        "Tab" => Ok(KeyCode::Tab),
        "Underline" => Ok(KeyCode::Underline),
        "Unlabeled" => Ok(KeyCode::Unlabeled),
        "VolumeDown" => Ok(KeyCode::VolumeDown),
        "VolumeUp" => Ok(KeyCode::VolumeUp),
        "Wake" => Ok(KeyCode::Wake),
        "WebBack" => Ok(KeyCode::WebBack),
        "WebFavorites" => Ok(KeyCode::WebFavorites),
        "WebForward" => Ok(KeyCode::WebForward),
        "WebHome" => Ok(KeyCode::WebHome),
        "WebRefresh" => Ok(KeyCode::WebRefresh),
        "WebSearch" => Ok(KeyCode::WebSearch),
        "WebStop" => Ok(KeyCode::WebStop),
        "Yen" => Ok(KeyCode::Yen),
        "Copy" => Ok(KeyCode::Copy),
        "Paste" => Ok(KeyCode::Paste),
        "Cut" => Ok(KeyCode::Cut),
        _ => { error!("No keycode registered with value {value}"); Ok(KeyCode::Escape) }
    }
}