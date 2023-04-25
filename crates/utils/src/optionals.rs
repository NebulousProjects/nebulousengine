use crate::enums::*;

use bevy::{prelude::*, render::{camera::Viewport, view::ColorGrading}, core_pipeline::tonemapping::DebandDither};
use json::JsonValue::{self};
use std::{u32, ops::Range};

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

pub fn optional_color_default(json_container: &JsonValue, target: &str, default: Color) -> Color {
    if json_container.has_key(target) { 
        let value = json_container[target].as_str().unwrap();
        return Color::rgb(
            u32::from_str_radix(&value[1..3], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[3..5], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[5..7], 16).unwrap() as f32 / 255.0
        ).into();
    } else {
        return default
    }
}

pub fn optional_viewport(json: &JsonValue, target: &str) -> Option<Viewport> {
    if json.has_key(target) {
        let value = &json[target];
        Some(
            Viewport {
                physical_position: optional_uvec2(value, "position"),
                physical_size: optional_uvec2(value, "size"),
                depth: optional_range_f32(value, "depth", 0.0 .. 1.0)
            }
        )
    } else { None }
}

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

pub fn optional_deband_dither(json: &JsonValue, target: &str) -> DebandDither {
    return if json.has_key(target) {
        if json[target].as_bool().unwrap_or(true) { DebandDither::Enabled } 
        else { DebandDither::Disabled }
    } else { DebandDither::default() }
}

pub fn optional_color_grading(json: &JsonValue, target: &str) -> ColorGrading {
    return if json.has_key(target) {
        let value = &json[target];
        ColorGrading {
            exposure: optional_f32(value, "exposure", 0.0),
            gamma: optional_f32(value, "gamma", 1.0),
            pre_saturation: optional_f32(value, "pre_saturation", 1.0),
            post_saturation: optional_f32(value, "post_saturation", 1.0)
        }
    } else { ColorGrading::default() }
}

pub fn optional_transform(json_container: &JsonValue, target: &str) -> Transform {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        let mut transform = Transform {
            translation: optional_vec3(value, "position", Vec3::ZERO),
            rotation: optional_quat(value, "rotation", Quat::default()),
            scale: optional_vec3(value, "scale", Vec3::ONE)
        };

        if value.has_key("look_at") {
            let look_at = &value["look_at"];
            transform.look_at(
                optional_vec3(look_at, "at", Vec3::ZERO), 
                optional_vec3(look_at, "axis", Vec3::ZERO)
            )
        }

        transform
    } else { Transform::default() }
}

pub fn optional_quat(json_container: &JsonValue, target: &str, default: Quat) -> Quat {
    return if json_container.has_key(target) {
        let value = json_container[target].clone();
        if value.is_array() {
            if value.len() == 4 {
                Quat::from_xyzw(
                    f32_default(&value[0], default.x),
                    f32_default(&value[1], default.y),
                    f32_default(&value[2], default.z),
                    f32_default(&value[3], default.w)
                )
            } else if value.len() == 3 {
                Quat::from_scaled_axis(
                    Vec3 { 
                        x: f32_default(&value[0], 0.0) * 0.0174532925199, 
                        y: f32_default(&value[1], 0.0) * 0.0174532925199, 
                        z: f32_default(&value[2], 0.0) * 0.0174532925199 
                    }
                )
            } else { default }
        } else if value.is_object() {
            if value.has_key("w") {
                Quat::from_xyzw(
                    optional_f32(&value, "x", default.x),
                    optional_f32(&value, "y", default.y),
                    optional_f32(&value, "z", default.z),
                    optional_f32(&value, "w", default.w),
                )
            } else {
                Quat::from_scaled_axis(
                    Vec3 { 
                        x: optional_f32(&value, "x", 0.0) * 0.0174532925199, 
                        y: optional_f32(&value, "y", 0.0) * 0.0174532925199, 
                        z: optional_f32(&value, "z", 0.0) * 0.0174532925199 
                    }
                )
            }
        } else { default }
    } else {
        default
    }
}

pub fn optional_vec3(json_container: &JsonValue, target: &str, default: Vec3) -> Vec3 {
    return if json_container.has_key(target) {
        let value = json_container[target].clone();
        if value.is_array() && value.len() == 3 {
            return Vec3 {
                x: f32_default(&value[0], default.x),
                y: f32_default(&value[1], default.y),
                z: f32_default(&value[2], default.z)
            }
        } else if value.is_object() {
            return Vec3 {
                x: optional_f32(&value, "x", default.x),
                y: optional_f32(&value, "y", default.y),
                z: optional_f32(&value, "z", default.z),
            }
        } else { return default }
    } else {
        default
    }
}

pub fn optional_uvec2(json: &JsonValue, target: &str) -> UVec2 {
    return if json.has_key(target) {
        let value = json[target].clone();
        if value.is_array() && value.len() == 2 {
            UVec2 {
                x: value[0].as_u32().unwrap_or(0),
                y: value[1].as_u32().unwrap_or(0)
            }
        } else if value.is_object() {
            UVec2 {
                x: optional_u32(json, "x", 0),
                y: optional_u32(json, "y", 0)
            }
        } else { UVec2 { x: 0, y: 0 } }
    } else { UVec2 { x: 0, y: 0 } }
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

pub fn optional_range_f32(json: &JsonValue, target: &str, default: Range<f32>) -> Range<f32> {
    return if json.has_key(target) {
        let value = &json[target];
        let start = optional_f32(value, "start", default.start);
        let end = optional_f32(value, "end", default.end);
        start .. end
    } else { default }
}

pub fn optional_isize(json: &JsonValue, target: &str, default: isize) -> isize {
    return if json.has_key(target) {
        json[target].as_isize().unwrap_or(default)
    } else { default }
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
    } else { default }
}

pub fn optional_u32(json: &JsonValue, target: &str, default: u32) -> u32 {
    return if json.has_key(target) {
        json[target].as_u32().unwrap_or(default)
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

pub fn optional_image(json_container: &JsonValue, asset_server: &Res<AssetServer>, target: &str) -> UiImage {
    return if json_container.has_key(target) {
        let image_json = &json_container[target];
        UiImage {
            texture: asset_server.load(optional_string(image_json, "path")),
            flip_x: optional_bool(image_json, "flip_x", false),
            flip_y: optional_bool(image_json, "flip_y", false),
        }
    } else { UiImage::default() }
}

pub fn optional_font(json_container: &JsonValue, asset_server: &Res<AssetServer>, target: &str) -> Handle<Font> {
    return asset_server.load(optional_string(json_container, target));
}

pub fn optional_text_style(json_container: &JsonValue, asset_server: &Res<AssetServer>, target: &str) -> TextStyle {
    return if json_container.has_key(target) {
        let root = &json_container[target];
        TextStyle {
            font: optional_font(root, asset_server, "font"),
            font_size: optional_f32(root, "font_size", 20.0),
            color: optional_color_default(root, "color", Color::WHITE)
        }
    } else { TextStyle::default() }
}

pub fn text_section(root: &JsonValue, asset_server: &Res<AssetServer>) -> TextSection {
    TextSection { 
        value: optional_string(root, "text").to_string(), 
        style: optional_text_style(root, asset_server, "style") 
    }
}

pub fn optional_text_sections(json_container: &JsonValue, asset_server: &Res<AssetServer>, target: &str) -> Vec<TextSection> {
    return if json_container.has_key(target) {
        let array = &json_container[target];
        if array.is_array() {
            let mut vec = Vec::new();

            for i in 0 .. array.len() {
                let element = &array[i];
                vec.push(text_section(element, asset_server));
            }

            return vec;
        } else { Vec::new() }
    } else { Vec::new() }
}

pub fn optional_text(json_container: &JsonValue, asset_server: &Res<AssetServer>, target: &str) -> Text {
    return if json_container.has_key(target) {
        let value = &json_container[target];
        Text {
            sections: optional_text_sections(value, asset_server, "sections"),
            alignment: text_alignment(optional_string(value, "alignment")),
            linebreak_behaviour: break_line_on(optional_string(value, "linebreak_behaviour"))
        }
    } else { Text::default() }
}