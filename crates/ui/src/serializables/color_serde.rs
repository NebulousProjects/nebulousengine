use bevy::prelude::*;
use serde::*;

// intermeidiate for deserializing colors
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ColorInter {
    StringInter(String),
    ColorInter(Color)
}

// serialize color
pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    color.serialize(serializer)
}

// deserialize color
pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error> where D: Deserializer<'de> {
    // deserialize to intermeidate
    let inter = ColorInter::deserialize(deserializer);
    if inter.is_err() { return Err(inter.err().unwrap()) }
    let inter = inter.unwrap();

    // deserialize inter
    let color = match inter {
        ColorInter::ColorInter(color) => color,
        ColorInter::StringInter(string) => {
            match string.as_str() {
                "white" => Color::WHITE,
                "black" => Color::BLACK,
                _ => Color::PINK
            }
        }
    };
    Ok(color)
}