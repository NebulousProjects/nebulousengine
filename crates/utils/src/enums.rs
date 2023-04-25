use bevy::{prelude::{*, shape::CapsuleUvProfile}, ui::FocusPolicy, text::BreakLineOn, core_pipeline::tonemapping::Tonemapping};

pub fn display(target: &str) -> Display {
    return match target {
        "flex" => Display::Flex,
        "none" => Display::None,
        _ => Display::DEFAULT
    }
}

pub fn position_type(target: &str) -> PositionType {
    return match target {
        "absolute" => PositionType::Absolute,
        "relative" => PositionType::Relative,
        _ => PositionType::default()
    }
}

pub fn direction(target: &str) -> Direction {
    return match target {
        "inherit" => Direction::Inherit,
        "left_to_right" => Direction::LeftToRight,
        "right_to_left" => Direction::RightToLeft,
        _ => Direction::DEFAULT
    }
}

pub fn flex_direction(target: &str) -> FlexDirection {
    return match target {
        "row" => FlexDirection::Row,
        "row_reverse" => FlexDirection::RowReverse,
        "column" => FlexDirection::Column,
        "column_reverse" => FlexDirection::ColumnReverse,
        _ => FlexDirection::DEFAULT
    }
}

pub fn flex_wrap(target: &str) -> FlexWrap {
    return match target {
        "no_wrap" => FlexWrap::NoWrap,
        "wrap" => FlexWrap::Wrap,
        "wrap_reverse" => FlexWrap::WrapReverse,
        _ => FlexWrap::default()
    }
}

pub fn align_items(target: &str) -> AlignItems {
    return match target {
        "start" => AlignItems::Start,
        "end" => AlignItems::End,
        "flex_start" => AlignItems::FlexStart,
        "flex_end" => AlignItems::FlexEnd,
        "center" => AlignItems::Center,
        "baseline" => AlignItems::Baseline,
        "stretch" => AlignItems::Stretch,
        _ => AlignItems::DEFAULT
    }
}

pub fn align_self(target: &str) -> AlignSelf {
    return match target {
        "auto" => AlignSelf::Auto,
        "start" => AlignSelf::Start,
        "end" => AlignSelf::End,
        "flex_start" => AlignSelf::FlexStart,
        "flex_end" => AlignSelf::FlexEnd,
        "center" => AlignSelf::Center,
        "baseline" => AlignSelf::Baseline,
        "stretch" => AlignSelf::Stretch,
        _ => AlignSelf::DEFAULT
    }
}

pub fn align_content(target: &str) -> AlignContent {
    return match target {
        "start" => AlignContent::Start,
        "end" => AlignContent::End,
        "flex_start" => AlignContent::FlexStart,
        "flex_end" => AlignContent::FlexEnd,
        "center" => AlignContent::Center,
        "stretch" => AlignContent::Stretch,
        "space_between" => AlignContent::SpaceBetween,
        "space_around" => AlignContent::SpaceAround,
        "space_evenly" => AlignContent::SpaceEvenly,
        _ => AlignContent::DEFAULT
    }
}

pub fn justify_content(target: &str) -> JustifyContent {
    return match target {
        "start" => JustifyContent::Start,
        "end" => JustifyContent::End,
        "flex_start" => JustifyContent::FlexStart,
        "flex_end" => JustifyContent::FlexEnd,
        "center" => JustifyContent::Center,
        "space_between" => JustifyContent::SpaceBetween,
        "space_around" => JustifyContent::SpaceAround,
        "space_evenly" => JustifyContent::SpaceEvenly,
        _ => JustifyContent::DEFAULT
    }
}

pub fn focus_policy(target: &str) -> FocusPolicy {
    return match target {
        "block" => FocusPolicy::Block,
        "pass" => FocusPolicy::Pass,
        _ => FocusPolicy::default()
    }
}

pub fn visibility(target: &str) -> Visibility {
    return match target {
        "inherited" => Visibility::Inherited,
        "hidden" => Visibility::Hidden,
        "visible" => Visibility::Visible,
        _ => Visibility::default()
    }
}

pub fn overflow(target: &str) -> Overflow {
    return match target {
        "visible" => Overflow::Visible,
        "hidden" => Overflow::Hidden,
        _ => Overflow::DEFAULT
    }
}

pub fn break_line_on(target: &str) -> BreakLineOn {
    return match target {
        "word_boundary" => BreakLineOn::WordBoundary,
        "any_character" => BreakLineOn::AnyCharacter,
        _ => BreakLineOn::WordBoundary
    }
}

pub fn text_alignment(target: &str) -> TextAlignment {
    return match target {
        "left" => TextAlignment::Left,
        "center" => TextAlignment::Center,
        "right" => TextAlignment::Right,
        _ => TextAlignment::Center
    }
}

pub fn projection(target: &str) -> Projection {
    return match target {
        "perspective" => Projection::Perspective(PerspectiveProjection::default()),
        "orthographic" => Projection::Orthographic(OrthographicProjection::default()),
        _ => Projection::default()
    }
}

pub fn tonemapping(target: &str) -> Tonemapping {
    return match target {
        "none" => Tonemapping::None,
        "reinhard" => Tonemapping::Reinhard,
        "reinhard_luminance" => Tonemapping::ReinhardLuminance,
        "aces_fitted" => Tonemapping::AcesFitted,
        "agx" => Tonemapping::AgX,
        "somewhat_boring_display_transform" => Tonemapping::SomewhatBoringDisplayTransform,
        "tony_mc_mapface" => Tonemapping::TonyMcMapface,
        "blender_filmic" => Tonemapping::BlenderFilmic,
        _ => Tonemapping::default()
    }
}

pub fn capsule_uv_mapping(target: &str) -> CapsuleUvProfile {
    return match target {
        "aspect" => CapsuleUvProfile::Aspect,
        "uniform" => CapsuleUvProfile::Uniform,
        "fixed" => CapsuleUvProfile::Fixed,
        _ => CapsuleUvProfile::default()
    }
}

pub fn val(value: &str) -> Val {
    // if passed value is auto or undefined, return corresponding enum value
    if value == "undefined" || value == "" { return Val::Undefined }
    else if value == "auto" { return Val::Auto }
    // if the passed value is a percent, attempt to convert it to a float and default to 0 if attempt fails
    else if value.ends_with('%') {
        return Val::Percent(convert_string_to_f32_with_default(&value[0..(value.len() - 1)], 0.0))
    }
    // otherwise, attempt to convert value to float and default to 0 if necessary
    else { return Val::Px(convert_string_to_f32_with_default(value, 0.0)) }
}

pub fn zindex(input: &str) -> ZIndex {
    // split input value into components, if not two, return default
    let value: Vec<&str> = input.split(".", ).collect();
    if value.len() != 2 { return ZIndex::default() }

    // attempt to convert number (value 1), if fails, return default
    let num_result = value[0].parse::<i32>();
    if num_result.is_err() { return ZIndex::default() }
    let num = num_result.unwrap();

    // match type (value 2) and return value
    let type_str = value[1].to_lowercase();
    return match type_str.as_str() {
        "local" => ZIndex::Local(num),
        "l" => ZIndex::Local(num),
        "global" => ZIndex::Global(num),
        "g" => ZIndex::Global(num),
        _ => ZIndex::default()
    }
}

fn convert_string_to_f32_with_default(value: &str, default: f32) -> f32 {
    return value.parse::<f32>().unwrap_or(default);
}