use bevy::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ValInter {
    Value(f32),
    String(String)
}

pub fn serialize<S>(val: &Val, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    // convert to intermediate
    let intermediate = match val {
        Val::Auto => ValInter::String("auto".to_string()),
        Val::Px(value) => ValInter::Value(*value),
        Val::Percent(value) => ValInter::String(format!("{}%", value)),
        Val::Vw(value) => ValInter::String(format!("{}w", value)),
        Val::Vh(value) => ValInter::String(format!("{}h", value)),
        Val::VMin(value) => ValInter::String(format!("{}m", value)),
        Val::VMax(value) => ValInter::String(format!("{}M", value)),
    };

    // serialize and pass back
    intermediate.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Val, D::Error> where D: Deserializer<'de> {
    // do initial deserialize
    let val_inter = ValInter::deserialize(deserializer);

    return if val_inter.is_ok() {
        match val_inter.unwrap() {
            ValInter::Value(value) => Ok(Val::Px(value)),
            ValInter::String(value) => {
                // if auto, return auto
                if value == "auto" { return Ok(Val::Auto) }

                // split value into its type and its f32 value
                let mut chars = value.chars();
                let value_type = chars.next_back().unwrap();
                let value = chars.as_str().parse::<f32>();

                // make sure value parsed properly
                if value.is_err() { println!("Value failed! {}", value.err().unwrap()); return Ok(Val::Px(0.)) }
                let value = value.unwrap();

                // match value type to expected output
                match value_type {
                    '%' => Ok(Val::Percent(value)),
                    'w' => Ok(Val::Vw(value)),
                    'h' => Ok(Val::Vh(value)),
                    'm' => Ok(Val::VMin(value)),
                    'M' => Ok(Val::VMax(value)),
                    _ => Ok(Val::Px(0.))
                }
            },
        }
    } else {
        Err(val_inter.err().unwrap())
    }
}