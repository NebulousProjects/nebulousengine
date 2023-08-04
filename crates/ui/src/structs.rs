use bevy::{ui::*, prelude::{Color, NodeBundle, ButtonBundle, TextBundle}, reflect::{TypeUuid, TypePath}, text::{Text, TextStyle}, ecs::system::EntityCommands};
use serde::*;

use crate::UiID;

mod color_serde;
mod uirect_serde;
mod val_serde;
mod zindex_serde;

#[derive(Serialize, Deserialize, TypePath, TypeUuid)]
#[serde(tag = "type")]
#[uuid = "cf1d2afd-c9ce-4c89-83ab-6473873f9398"]
pub enum UiElement {
    Node {
        // style
        #[serde(default)]
        display: Display,
        #[serde(default)]
        position_type: PositionType,
        #[serde(default)]
        overflow: Overflow,
        #[serde(default)]
        direction: Direction,
        #[serde(default = "default_val_auto", with = "val_serde")]
        left: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        right: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        top: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        bottom: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_height: Val,
        #[serde(default)]
        aspect_ratio: Option<f32>,
        #[serde(default)]
        align_items: AlignItems,
        #[serde(default)]
        justify_items: JustifyItems,
        #[serde(default)]
        align_self: AlignSelf,
        #[serde(default)]
        justify_self: JustifySelf,
        #[serde(default)]
        align_content: AlignContent,
        #[serde(default)]
        justify_content: JustifyContent,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        margin: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        padding: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        border: UiRect,
        #[serde(default)]
        flex_direction: FlexDirection,
        #[serde(default)]
        flex_wrap: FlexWrap,
        #[serde(default = "default_float_0")]
        flex_grow: f32,
        #[serde(default = "default_float_1")]
        flex_shrink: f32,
        #[serde(default = "default_val_auto", with = "val_serde")]
        flex_basis: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        row_gap: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        column_gap: Val,
        #[serde(default)]
        grid_auto_flow: GridAutoFlow,
        // grid_template_rows: Vec<RepeatedGridTrack, Global>,
        // grid_template_columns: Vec<RepeatedGridTrack, Global>,
        // grid_auto_rows: Vec<GridTrack, Global>,
        // grid_auto_columns: Vec<GridTrack, Global>,
        #[serde(default)]
        grid_row: GridPlacement,
        #[serde(default)]
        grid_column: GridPlacement,

        // bundle
        #[serde(default = "default_transparent", with = "color_serde")]
        background_color: Color,
        #[serde(default = "default_black", with = "color_serde")]
        border_color: Color,
        #[serde(default)]
        focus_policy: FocusPolicy,
        #[serde(default = "default_local_0", with = "zindex_serde")]
        z_index: ZIndex,

        // other
        #[serde(default)]
        id: Option<String>,
        #[serde(default = "default_children")]
        children: Vec<UiElement>
    },
    Button {
        // style
        #[serde(default)]
        display: Display,
        #[serde(default)]
        position_type: PositionType,
        #[serde(default)]
        overflow: Overflow,
        #[serde(default)]
        direction: Direction,
        #[serde(default = "default_val_auto", with = "val_serde")]
        left: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        right: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        top: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        bottom: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_height: Val,
        #[serde(default)]
        aspect_ratio: Option<f32>,
        #[serde(default)]
        align_items: AlignItems,
        #[serde(default)]
        justify_items: JustifyItems,
        #[serde(default)]
        align_self: AlignSelf,
        #[serde(default)]
        justify_self: JustifySelf,
        #[serde(default)]
        align_content: AlignContent,
        #[serde(default)]
        justify_content: JustifyContent,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        margin: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        padding: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        border: UiRect,
        #[serde(default)]
        flex_direction: FlexDirection,
        #[serde(default)]
        flex_wrap: FlexWrap,
        #[serde(default = "default_float_0")]
        flex_grow: f32,
        #[serde(default = "default_float_1")]
        flex_shrink: f32,
        #[serde(default = "default_val_auto", with = "val_serde")]
        flex_basis: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        row_gap: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        column_gap: Val,
        #[serde(default)]
        grid_auto_flow: GridAutoFlow,
        // grid_template_rows: Vec<RepeatedGridTrack, Global>,
        // grid_template_columns: Vec<RepeatedGridTrack, Global>,
        // grid_auto_rows: Vec<GridTrack, Global>,
        // grid_auto_columns: Vec<GridTrack, Global>,
        #[serde(default)]
        grid_row: GridPlacement,
        #[serde(default)]
        grid_column: GridPlacement,

        // bundle
        #[serde(default = "default_transparent", with = "color_serde")]
        background_color: Color,
        #[serde(default = "default_black", with = "color_serde")]
        border_color: Color,
        #[serde(default)]
        focus_policy: FocusPolicy,
        #[serde(default = "default_local_0", with = "zindex_serde")]
        z_index: ZIndex,

        // other
        #[serde(default)]
        id: Option<String>,
        #[serde(default = "default_children")]
        children: Vec<UiElement>
    },
    Text {
        // style
        #[serde(default)]
        display: Display,
        #[serde(default)]
        position_type: PositionType,
        #[serde(default)]
        overflow: Overflow,
        #[serde(default)]
        direction: Direction,
        #[serde(default = "default_val_auto", with = "val_serde")]
        left: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        right: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        top: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        bottom: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        min_height: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_width: Val,
        #[serde(default = "default_val_auto", with = "val_serde")]
        max_height: Val,
        #[serde(default)]
        aspect_ratio: Option<f32>,
        #[serde(default)]
        align_items: AlignItems,
        #[serde(default)]
        justify_items: JustifyItems,
        #[serde(default)]
        align_self: AlignSelf,
        #[serde(default)]
        justify_self: JustifySelf,
        #[serde(default)]
        align_content: AlignContent,
        #[serde(default)]
        justify_content: JustifyContent,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        margin: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        padding: UiRect,
        #[serde(with = "uirect_serde", default = "default_uirect_0")]
        border: UiRect,
        #[serde(default)]
        flex_direction: FlexDirection,
        #[serde(default)]
        flex_wrap: FlexWrap,
        #[serde(default = "default_float_0")]
        flex_grow: f32,
        #[serde(default = "default_float_1")]
        flex_shrink: f32,
        #[serde(default = "default_val_auto", with = "val_serde")]
        flex_basis: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        row_gap: Val,
        #[serde(default = "default_val_0", with = "val_serde")]
        column_gap: Val,
        #[serde(default)]
        grid_auto_flow: GridAutoFlow,
        // grid_template_rows: Vec<RepeatedGridTrack, Global>,
        // grid_template_columns: Vec<RepeatedGridTrack, Global>,
        // grid_auto_rows: Vec<GridTrack, Global>,
        // grid_auto_columns: Vec<GridTrack, Global>,
        #[serde(default)]
        grid_row: GridPlacement,
        #[serde(default)]
        grid_column: GridPlacement,

        // bundle
        #[serde(default = "default_transparent", with = "color_serde")]
        background_color: Color,
        #[serde(default)]
        focus_policy: FocusPolicy,
        #[serde(default = "default_local_0", with = "zindex_serde")]
        z_index: ZIndex,
        #[serde(default)]
        text: String,
        #[serde(default = "default_black", with = "color_serde")]
        color: Color,
        #[serde(default = "default_font_size")]
        font_size: f32,

        // other
        #[serde(default)]
        id: Option<String>,
        #[serde(default = "default_children")]
        children: Vec<UiElement>
    }
}

impl UiElement {
    pub fn insert_bundle(&self, commands: &mut EntityCommands) {
        match self {
            UiElement::Node { 
                display, position_type, overflow, direction,
                left, right, top, bottom, width, height, 
                min_width, min_height, max_width, max_height, 
                aspect_ratio, align_items, justify_items, 
                align_self, justify_self, align_content, justify_content, 
                margin, padding, border, flex_direction, flex_wrap, 
                flex_grow, flex_shrink, flex_basis, row_gap, column_gap, 
                grid_auto_flow: _, grid_row, grid_column, 
                background_color, border_color, focus_policy, 
                z_index, id, children: _ 
            } => {
                commands.insert(NodeBundle {
                    style: Style {
                        display: *display,
                        position_type: *position_type,
                        overflow: *overflow,
                        direction: *direction,
                        left: *left,
                        right: *right,
                        top: *top,
                        bottom: *bottom,
                        width: *width,
                        height: *height,
                        min_width: *min_width,
                        min_height: *min_height,
                        max_width: *max_width,
                        max_height: *max_height,
                        aspect_ratio: *aspect_ratio,
                        align_items: *align_items,
                        justify_items: *justify_items,
                        align_self: *align_self,
                        justify_self: *justify_self,
                        align_content: *align_content,
                        justify_content: *justify_content,
                        margin: *margin,
                        padding: *padding,
                        border: *border,
                        flex_direction: *flex_direction,
                        flex_wrap: *flex_wrap,
                        flex_grow: *flex_grow,
                        flex_shrink: *flex_shrink,
                        flex_basis: *flex_basis,
                        row_gap: *row_gap,
                        column_gap: *column_gap,
                        grid_row: *grid_row,
                        grid_column: *grid_column,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(*background_color),
                    border_color: BorderColor(*border_color),
                    focus_policy: *focus_policy,
                    z_index: *z_index,
                    ..Default::default()
                });
                if id.is_some() { commands.insert(UiID(id.clone().unwrap())); }
            },
            UiElement::Button { 
                display, position_type, overflow, direction, 
                left, right, top, bottom, width, height, 
                min_width, min_height, max_width, max_height, aspect_ratio, 
                align_items, justify_items, align_self, justify_self, 
                align_content, justify_content, margin, padding, border, 
                flex_direction, flex_wrap, flex_grow, flex_shrink, flex_basis, 
                row_gap, column_gap, grid_auto_flow: _, grid_row, 
                grid_column, background_color, border_color, focus_policy, 
                z_index, id, children: _ 
            } => {
                commands.insert(ButtonBundle {
                    style: Style {
                        display: *display,
                        position_type: *position_type,
                        overflow: *overflow,
                        direction: *direction,
                        left: *left,
                        right: *right,
                        top: *top,
                        bottom: *bottom,
                        width: *width,
                        height: *height,
                        min_width: *min_width,
                        min_height: *min_height,
                        max_width: *max_width,
                        max_height: *max_height,
                        aspect_ratio: *aspect_ratio,
                        align_items: *align_items,
                        justify_items: *justify_items,
                        align_self: *align_self,
                        justify_self: *justify_self,
                        align_content: *align_content,
                        justify_content: *justify_content,
                        margin: *margin,
                        padding: *padding,
                        border: *border,
                        flex_direction: *flex_direction,
                        flex_wrap: *flex_wrap,
                        flex_grow: *flex_grow,
                        flex_shrink: *flex_shrink,
                        flex_basis: *flex_basis,
                        row_gap: *row_gap,
                        column_gap: *column_gap,
                        grid_row: *grid_row,
                        grid_column: *grid_column,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(*background_color),
                    border_color: BorderColor(*border_color),
                    focus_policy: *focus_policy,
                    z_index: *z_index,
                    ..Default::default()
                });
                if id.is_some() { commands.insert(UiID(id.clone().unwrap())); }
            },
            UiElement::Text { 
                display, position_type, overflow, direction, 
                left, right, top, bottom, width, height, 
                min_width, min_height, max_width, max_height, aspect_ratio, 
                align_items, justify_items, align_self, justify_self, 
                align_content, justify_content, margin, padding, border, 
                flex_direction, flex_wrap, flex_grow, flex_shrink, flex_basis, 
                row_gap, column_gap, grid_auto_flow: _, grid_row, 
                grid_column, background_color, focus_policy, 
                z_index, id, children: _, text, color, font_size, 
            } => {
                commands.insert(TextBundle {
                    style: Style {
                        display: *display,
                        position_type: *position_type,
                        overflow: *overflow,
                        direction: *direction,
                        left: *left,
                        right: *right,
                        top: *top,
                        bottom: *bottom,
                        width: *width,
                        height: *height,
                        min_width: *min_width,
                        min_height: *min_height,
                        max_width: *max_width,
                        max_height: *max_height,
                        aspect_ratio: *aspect_ratio,
                        align_items: *align_items,
                        justify_items: *justify_items,
                        align_self: *align_self,
                        justify_self: *justify_self,
                        align_content: *align_content,
                        justify_content: *justify_content,
                        margin: *margin,
                        padding: *padding,
                        border: *border,
                        flex_direction: *flex_direction,
                        flex_wrap: *flex_wrap,
                        flex_grow: *flex_grow,
                        flex_shrink: *flex_shrink,
                        flex_basis: *flex_basis,
                        row_gap: *row_gap,
                        column_gap: *column_gap,
                        grid_row: *grid_row,
                        grid_column: *grid_column,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        text, 
                        TextStyle { font_size: *font_size, color: *color, ..Default::default() }
                    ),
                    background_color: BackgroundColor(*background_color),
                    focus_policy: *focus_policy,
                    z_index: *z_index,
                    ..Default::default()
                });
                if id.is_some() { commands.insert(UiID(id.clone().unwrap())); }
            }
        };
    }

    pub fn get_children(&self) -> &Vec<UiElement> {
        match self {
            UiElement::Node { children, .. } => children,
            UiElement::Button { children, .. } => children,
            UiElement::Text { children, .. } => children,
        }
    }
}

// defaults
fn default_float_0() -> f32 { 0. }
fn default_float_1() -> f32 { 1. }
fn default_font_size() -> f32 { 25. }
fn default_transparent() -> Color { Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. } }
fn default_black() -> Color { Color::BLACK }
fn default_uirect_0() -> UiRect { UiRect::all(Val::Px(0.)) }
fn default_val_auto() -> Val { Val::Auto }
fn default_val_0() -> Val { Val::Px(0.0) }
fn default_local_0() -> ZIndex { ZIndex::Local(0) }
fn default_children() -> Vec<UiElement> { Vec::new() }