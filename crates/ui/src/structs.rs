use bevy::{ui::*, prelude::{Color, NodeBundle, ButtonBundle, TextBundle}, reflect::{TypeUuid, TypePath}, text::{Text, TextStyle}, ecs::system::EntityCommands};
use serde::*;
use serde_json::Value;

use crate::{UiID, UiData};

mod color_serde;
mod uirect_serde;
mod val_serde;
mod zindex_serde;

#[derive(Serialize, Deserialize, TypePath, TypeUuid, Debug, Default)]
#[uuid = "cf1d2afd-c9ce-4c89-83ab-6473873f9398"]
pub struct UiElement {
    #[serde(rename = "type")]
    pub subtype: UiElementType,

    // style
    #[serde(default)]
    pub display: Display,
    #[serde(default)]
    pub position_type: PositionType,
    #[serde(default)]
    pub overflow: Overflow,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub left: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub right: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub top: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub bottom: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub width: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub height: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub min_width: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub min_height: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub max_width: Val,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub max_height: Val,
    #[serde(default)]
    pub aspect_ratio: Option<f32>,
    #[serde(default)]
    pub align_items: AlignItems,
    #[serde(default)]
    pub justify_items: JustifyItems,
    #[serde(default)]
    pub align_self: AlignSelf,
    #[serde(default)]
    pub justify_self: JustifySelf,
    #[serde(default)]
    pub align_content: AlignContent,
    #[serde(default)]
    pub justify_content: JustifyContent,
    #[serde(with = "uirect_serde", default = "default_uirect_0")]
    pub margin: UiRect,
    #[serde(with = "uirect_serde", default = "default_uirect_0")]
    pub padding: UiRect,
    #[serde(with = "uirect_serde", default = "default_uirect_0")]
    pub border: UiRect,
    #[serde(default)]
    pub flex_direction: FlexDirection,
    #[serde(default)]
    pub flex_wrap: FlexWrap,
    #[serde(default = "default_float_0")]
    pub flex_grow: f32,
    #[serde(default = "default_float_1")]
    pub flex_shrink: f32,
    #[serde(default = "default_val_auto", with = "val_serde")]
    pub flex_basis: Val,
    #[serde(default = "default_val_0", with = "val_serde")]
    pub row_gap: Val,
    #[serde(default = "default_val_0", with = "val_serde")]
    pub column_gap: Val,
    #[serde(default)]
    pub grid_auto_flow: GridAutoFlow,
    // grid_template_rows: Vec<RepeatedGridTrack, Global>,
    // grid_template_columns: Vec<RepeatedGridTrack, Global>,
    // grid_auto_rows: Vec<GridTrack, Global>,
    // grid_auto_columns: Vec<GridTrack, Global>,
    #[serde(default)]
    pub grid_row: GridPlacement,
    #[serde(default)]
    pub grid_column: GridPlacement,

    // bundle
    #[serde(default = "default_transparent", with = "color_serde")]
    pub background_color: Color,
    #[serde(default = "default_black", with = "color_serde")]
    pub border_color: Color,
    #[serde(default)]
    pub focus_policy: FocusPolicy,
    #[serde(default = "default_local_0", with = "zindex_serde")]
    pub z_index: ZIndex,
    #[serde(default)]
    pub text: String,
    #[serde(default = "default_black", with = "color_serde")]
    pub color: Color,
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    // other
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)] //  = "default_children"
    pub children: Vec<UiElement>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum UiElementType {
    #[default]
    Node, 
    Text, 
    Button
}

impl UiElement {
    pub fn get_style(&self) -> Style {
        Style {
            display: self.display,
            position_type: self.position_type,
            overflow: self.overflow,
            direction: self.direction,
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            aspect_ratio: self.aspect_ratio,
            align_items: self.align_items,
            justify_items: self.justify_items,
            align_self: self.align_self,
            justify_self: self.justify_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            margin: self.margin,
            padding: self.padding,
            border: self.border,
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis,
            row_gap: self.row_gap,
            column_gap: self.column_gap,
            grid_row: self.grid_row,
            grid_column: self.grid_column,
            ..Default::default()
        }
    }

    pub fn insert_bundle(&self, commands: &mut EntityCommands) {
        // insert ui bundle based on sub type
        match self.subtype {
            // spawn node element (essentially a div)
            UiElementType::Node => {
                commands.insert(NodeBundle {
                    style: self.get_style(),
                    background_color: BackgroundColor(self.background_color),
                    border_color: BorderColor(self.border_color),
                    focus_policy: self.focus_policy,
                    z_index: self.z_index,
                    ..Default::default()
                });
            },

            // spawn a button element (clickable div)
            UiElementType::Button => {
                commands.insert(ButtonBundle {
                    style: self.get_style(),
                    background_color: BackgroundColor(self.background_color),
                    border_color: BorderColor(self.border_color),
                    focus_policy: self.focus_policy,
                    z_index: self.z_index,
                    ..Default::default()
                });
            },

            // spawn a text element
            UiElementType::Text => {
                commands.insert(TextBundle {
                    style: self.get_style(),
                    text: Text::from_section(
                        self.text.clone(), 
                        TextStyle { font_size: self.font_size, color: self.color, ..Default::default() }
                    ),
                    background_color: BackgroundColor(self.background_color),
                    focus_policy: self.focus_policy,
                    z_index: self.z_index,
                    ..Default::default()
                });
            },
        }

        // add id and data components
        if self.id.is_some() { commands.insert(UiID(self.id.clone().unwrap())); }
        if self.data.is_some() { commands.insert(UiData(self.data.clone().unwrap())); }
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
// fn default_children() -> Vec<UiElement<_>> { Vec::new() }
