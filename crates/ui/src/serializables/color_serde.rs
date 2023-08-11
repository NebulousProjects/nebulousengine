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
            match string.to_lowercase().as_str() {
                // rgb(94%, 97%, 100%)
                "alice_blue" => Color::ALICE_BLUE,
                // rgb(98%, 92%, 84%)
                "antique_white" => Color::ANTIQUE_WHITE,
                // rgb(49%, 100%, 83%)
                "aquamarine" => Color::AQUAMARINE,
                // rgb(94%, 100%, 100%)
                "azure" => Color::AZURE,
                // rgb(96%, 96%, 86%)
                "beige" => Color::BEIGE,
                // rgb(100%, 89%, 77%)
                "bisque" => Color::BISQUE,
                // rgb(0%, 0%, 0%)
                "black" => Color::BLACK,
                // rgb(0%, 0%, 100%)
                "blue" => Color::BLUE,
                // rgb(86%, 8%, 24%)
                "crimson" => Color::CRIMSON,
                // rgb(0%, 100%, 100%)
                "cyan" => Color::CYAN,
                // rgb(25%, 25%, 25%)
                "dark_gray" => Color::DARK_GRAY,
                // rgb(0%, 50%, 0%)
                "dark_green" => Color::DARK_GREEN,
                // rgb(100%, 0%, 100%)
                "fuchsia" => Color::FUCHSIA,
                // rgb(100%, 84%, 0%)
                "gold" => Color::GOLD,
                // rgb(50%, 50%, 50%)
                "gray" => Color::GRAY,
                // rgb(0%, 100%, 0%)
                "green" => Color::GREEN,
                // rgb(28%, 0%, 51%)
                "indigo" => Color::INDIGO,
                // rgb(20%, 80%, 20%)
                "lime_green" => Color::LIME_GREEN,
                // rgb(50%, 0%, 0%)
                "maroon" => Color::MAROON,
                // rgb(10%, 10%, 44%)
                "midnight_blue" => Color::MIDNIGHT_BLUE,
                // rgb(0%, 0%, 50%)
                "navy" => Color::NAVY,
                // rgba(0%, 0%, 0%, 0%)
                "none" => Color::NONE,
                // rgb(50%, 50%, 0%)
                "olive" => Color::OLIVE,
                // rgb(100%, 65%, 0%)
                "orange" => Color::ORANGE,
                // rgb(100%, 27%, 0%)
                "orange_red" => Color::ORANGE_RED,
                // rgb(100%, 8%, 57%)
                "pink" => Color::PINK,
                // rgb(50%, 0%, 50%)
                "purple" => Color::PURPLE,
                // rgb(100%, 0%, 0%)
                "red" => Color::RED,
                // rgb(98%, 50%, 45%)
                "salmon" => Color::SALMON,
                // rgb(18%, 55%, 34%)
                "sea_green" => Color::SEA_GREEN,
                // rgb(75%, 75%, 75%)
                "silver" => Color::SILVER,
                // rgb(0%, 50%, 50%)
                "teal" => Color::TEAL,
                // rgb(100%, 39%, 28%)
                "tomato" => Color::TOMATO,
                // rgb(25%, 88%, 82%)
                "turquoise" => Color::TURQUOISE,
                // rgb(93%, 51%, 93%)
                "violet" => Color::VIOLET,
                // rgb(100%, 100%, 100%)
                "white" => Color::WHITE,
                // rgb(100%, 100%, 0%)
                "yellow" => Color::YELLOW,
                // rgb(60%, 80%, 20%)
                "yellow_green" => Color::YELLOW_GREEN,
                _ => Color::PINK
            }
        }
    };
    Ok(color)
}