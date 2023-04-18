use crate::enum_utils::*;

use bevy::prelude::*;
use json::{ JsonValue };
use std::u32;

pub fn optional_style(json_container: &JsonValue, target: &str) -> Style {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        Style {
            display: display(optional_string(value, "display")),
            position_type: position_type(optional_string(value, "position_type")),
            direction: direction(optional_string(value, "direction")),
            flex_direction: flex_direction(optional_string(value, "flex_direction")),
            flex_wrap: flex_wrap(optional_string(value, "flex_wrap")),
            align_items: align_items(optional_string(value, "align_items")),
            align_self: align_self(optional_string(value, "align_self")),
            align_content: align_content(optional_string(value, "align_content")),
            justify_content: justify_content(optional_string(value, "justify_content")),
            position: optional_uirect(value, "position"),
            margin: optional_uirect(value, "margin"),
            padding: optional_uirect(value, "padding"),
            border: optional_uirect(value, "border"),
            flex_grow: optional_f32(value, "flex_grow", 0.0),
            flex_shrink: optional_f32(value, "flex_shrink", 0.0),
            flex_basis: optional_val(value, "flex_basis"),
            size: optional_size(value, "size"),
            max_size: optional_size(value, "max_size"),
            overflow: overflow(optional_string(value, "overflow")),
            gap: optional_size(value, "gap"),
            ..Default::default() // default for aspect ratio
        }
    } else { Style::DEFAULT }
}

pub fn optional_color(json_container: &JsonValue, target: &str) -> Color {
    if json_container.has_key(target) { 
        let value = json_container[target].as_str().unwrap();
        return Color::rgb(
            u32::from_str_radix(&value[1..3], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[3..5], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[5..7], 16).unwrap() as f32 / 255.0
        ).into();
    } else {
        return Color::NONE.into();
    }
}

// todo if just number, use ui rect all
pub fn optional_uirect(json_container: &JsonValue, target: &str) -> UiRect {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        if value.is_number() { UiRect::all(Val::Px(value.as_f32().unwrap())) }
        else if value.is_string() { UiRect::all(val(value.as_str().unwrap())) }
        else {
            UiRect {
                bottom: optional_val(value, "bottom"),
                top: optional_val(value, "top"),
                left: optional_val(value, "left"),
                right: optional_val(value, "right")
            }
        }
    } else { UiRect::DEFAULT }
}

pub fn optional_size(json_container: &JsonValue, target: &str) -> Size {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        Size {
            width: optional_val(value, "width"),
            height: optional_val(value, "height")
        }
    } else {
        Size::DEFAULT
    }
}

pub fn optional_val(json_container: &JsonValue, target: &str) -> Val {
    // if the given target is in the json container, parse, otherwise, return the default
    if json_container.has_key(target) {
        let value = &json_container[target];
        if value.is_number() { Val::Px(value.as_f32().unwrap()) }
        else { val(value.as_str().unwrap_or("")) }
    } else {
        return Val::DEFAULT;
    }
}

pub fn optional_transform(json_container: &JsonValue, target: &str) -> Transform {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        Transform {
            translation: optional_vec3(value, "position", Vec3::ZERO),
            rotation: optional_quat(value, "rotation", Quat::default()),
            scale: optional_vec3(value, target, Vec3::ONE)
        }
    } else { Transform::default() }
}

pub fn optional_quat(json_container: &JsonValue, target: &str, default: Quat) -> Quat {
    return if json_container.has_key(target) {
        let value = json_container[target].clone();
        if value.is_array() && value.len() == 4 {
            Quat::from_xyzw(
                f32_default(&value[0], default.x),
                f32_default(&value[1], default.y),
                f32_default(&value[2], default.z),
                f32_default(&value[3], default.w)
            )
        } else if value.is_object() {
            Quat::from_xyzw(
                optional_f32(&value, "x", default.x),
                optional_f32(&value, "y", default.y),
                optional_f32(&value, "z", default.z),
                optional_f32(&value, "w", default.w),
            )
        } else { default }
    } else {
        default
    }
}

pub fn optional_vec3(json_container: &JsonValue, target: &str, default: Vec3) -> Vec3 {
    return if json_container.has_key(target) {
        let value = json_container[target].clone();
        if value.is_array() && value.len() == 3 {
            Vec3 {
                x: f32_default(&value[0], default.x),
                y: f32_default(&value[1], default.y),
                z: f32_default(&value[2], default.z)
            }
        } else if value.is_object() {
            Vec3 {
                x: optional_f32(&value, "x", default.x),
                y: optional_f32(&value, "y", default.y),
                z: optional_f32(&value, "z", default.z),
            }
        } else { default }
    } else {
        default
    }
}

pub fn optional_calculated_size(json_container: &JsonValue, target: &str) -> CalculatedSize {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        CalculatedSize {
            size: Vec2 { 
                x: optional_f32(value, "width", 0.0), 
                y: optional_f32(value, "height", 0.0)
            }, preserve_aspect_ratio: optional_bool(value, "preserve_aspect_ratio", false)
        }
    } else { CalculatedSize::default() }
}

pub fn optional_string<'a>(json_container: &'a JsonValue, target: &'a str) -> &'a str {
    return if json_container.has_key(target) {
        json_container[target].as_str().unwrap_or("") // rust bullshit
    } else { "" }
}

pub fn f32_default(value: &JsonValue, default: f32) -> f32 {
    return value.as_f32().unwrap_or(default)
}

pub fn optional_f32(json_container: &JsonValue, target: &str, default: f32) -> f32 {
    return if json_container.has_key(target) {
        f32_default(&json_container[target], default)
        // json_container[target].clone().as_f32().unwrap_or(default) // rust bullshit
    } else { default }
}

pub fn optional_bool(json_container: &JsonValue, target: &str, default: bool) -> bool {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        if value.is_boolean() {
            value.as_bool().unwrap()
        } else if value.is_string() {
            value.as_str().unwrap() == "true"
        } else { default }
    } else { default }
}