use bevy::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum UiRectInter {
    Fast(
        #[serde(default = "default_val_0", with = "super::val_serde")]
        Val
    ),
    Slow {
        #[serde(default = "default_val_0", with = "super::val_serde")]
        left: Val,
        #[serde(default = "default_val_0", with = "super::val_serde")]
        right: Val,
        #[serde(default = "default_val_0", with = "super::val_serde")]
        top: Val,
        #[serde(default = "default_val_0", with = "super::val_serde")]
        bottom: Val
    }
}

pub fn serialize<S>(rect: &UiRect, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let inter = match rect.left == rect.right && rect.left == rect.bottom && rect.bottom == rect.top {
        true => UiRectInter::Fast(rect.left),
        false => UiRectInter::Slow { left: rect.left, right: rect.right, top: rect.top, bottom: rect.bottom }
    };
    inter.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<UiRect, D::Error> where D: Deserializer<'de> {
    // do initial deserialize
    let val_inter = UiRectInter::deserialize(deserializer);

    return if val_inter.is_ok() {
        match val_inter.unwrap() {
            UiRectInter::Fast(value) => Ok(UiRect::all(value)),
            UiRectInter::Slow { left, right, top, bottom } => Ok(UiRect { left, right, bottom, top }),
        }
    } else {
        Err(val_inter.err().unwrap())
    }
}

fn default_val_0() -> Val { Val::Px(0.0) }