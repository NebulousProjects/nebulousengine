use bevy::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ZIndexInter {
    Local(i32),
    Global(i32)
}

pub fn serialize<S>(input: &ZIndex, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let inter = match input {
        ZIndex::Local(value) => ZIndexInter::Local(*value),
        ZIndex::Global(value) => ZIndexInter::Global(*value)
    };
    inter.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ZIndex, D::Error> where D: Deserializer<'de> {
    let inter = ZIndexInter::deserialize(deserializer);
    if inter.is_err() { return Err(inter.err().unwrap()) }
    let inter = inter.unwrap();

    let zindex = match inter {
        ZIndexInter::Local(value) => ZIndex::Local(value),
        ZIndexInter::Global(value) => ZIndex::Global(value)
    };
    Ok(zindex)
}