use bevy::ui::{JustifyContent, Display, Direction, FlexDirection, PositionType, FlexWrap, AlignItems, AlignSelf, AlignContent, Overflow};

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

pub fn overflow(target: &str) -> Overflow {
    return match target {
        "visible" => Overflow::Visible,
        "hidden" => Overflow::Hidden,
        _ => Overflow::DEFAULT
    }
}