use bevy::prelude::*;
use serde_json::*;

use crate::{ui::UI, HoverColor, PressColor};

#[derive(Default, Debug, Clone, Copy)]
pub struct BorderInfo(pub UiRect, pub Color);

#[derive(Resource, Default, Debug, Clone)]
pub struct UINode {
    pub id: Option<String>,
    pub data: Option<Value>,

    pub ui: UI,
    pub style: Style,
    pub background_color: Color,
    pub border: Option<BorderInfo>,
    pub image: Option<String>,

    pub representation: Option<Entity>,
    pub children: Vec<UINode>,
    pub is_dirty: bool
}

impl UINode {
    pub fn mark_dirty(&mut self) -> &mut UINode { self.is_dirty = true; return self }

    // functions to update properties
    pub fn id(&mut self, id: impl Into<String>) -> &mut UINode { self.id = Some(id.into()); self.mark_dirty() }
    pub fn data(&mut self, data: Value) -> &mut UINode { self.data = Some(data); self.mark_dirty() }
    pub fn style(&mut self, style: Style) -> &mut UINode { self.style = style; self.mark_dirty() }
    pub fn bg(&mut self, color: Color) -> &mut UINode { self.background_color = color; self.mark_dirty() }
    pub fn border(&mut self, shape: UiRect, color: Color) -> &mut UINode { self.border = Some(BorderInfo(shape, color)); self.mark_dirty() }
    pub fn image(&mut self, path: impl Into<String>) -> &mut Self { self.image = Some(path.into()); self.bg(Color::WHITE) }

    // enum ez functions
    pub fn panel(&mut self) -> &mut UINode { self.add(UI::Panel) }
    pub fn scroll_panel(&mut self, flex_direction: FlexDirection) -> &mut UINode { self.add(UI::ScrollPanel { flex_direction }) }
    pub fn text(&mut self, text: impl Into<String>) -> &mut UINode { self.add(UI::Text { text: text.into() }) }
    pub fn text_area(&mut self, text_color: Color, font_size: f32) -> &mut Self { self.add(UI::TextArea { text_color, font_size, default_text: String::new(), ghost_text: String::new(), selected_bg: None, selected_border: None }) }
    pub fn button(&mut self) -> &mut UINode { self.add(UI::Button { hover_bg: None, press_bg: None }) }
    pub fn slider(&mut self, direction: FlexDirection, first: Color, second: Color, amount: f32) -> &mut Self { self.add(UI::Slider { direction, first, second, amount, moveable: false }) }

    // style ez functions
    pub fn display(&mut self, display: Display) -> &mut Self { self.style.display = display; self.mark_dirty() }
    pub fn position_type(&mut self, position_type: PositionType) -> &mut Self { self.style.position_type = position_type; self.mark_dirty() }
    pub fn overflow(&mut self, overflow: Overflow) -> &mut Self { self.style.overflow = overflow; self.mark_dirty() }
    pub fn direction(&mut self, direction: Direction) -> &mut Self { self.style.direction = direction; self.mark_dirty() }
    pub fn left(&mut self, left: Val) -> &mut Self { self.style.left = left; self.mark_dirty() }
    pub fn right(&mut self, right: Val) -> &mut Self { self.style.right = right; self.mark_dirty() }
    pub fn top(&mut self, top: Val) -> &mut Self { self.style.top = top; self.mark_dirty() }
    pub fn bottom(&mut self, bottom: Val) -> &mut Self { self.style.bottom = bottom; self.mark_dirty() }
    pub fn width(&mut self, width: Val) -> &mut Self { self.style.width = width; self.mark_dirty() }
    pub fn height(&mut self, height: Val) -> &mut Self { self.style.height = height; self.mark_dirty() }
    pub fn min_width(&mut self, min_width: Val) -> &mut Self { self.style.min_width = min_width; self.mark_dirty() }
    pub fn min_height(&mut self, min_height: Val) -> &mut Self { self.style.min_height = min_height; self.mark_dirty() }
    pub fn max_width(&mut self, max_width: Val) -> &mut Self { self.style.max_width = max_width; self.mark_dirty() }
    pub fn max_height(&mut self, max_height: Val) -> &mut Self { self.style.max_height = max_height; self.mark_dirty() }
    pub fn aspect_ratio(&mut self, aspect_ratio: Option<f32>) -> &mut Self { self.style.aspect_ratio = aspect_ratio; self.mark_dirty() }
    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Self { self.style.align_items = align_items; self.mark_dirty() }
    pub fn justify_items(&mut self, justify_items: JustifyItems) -> &mut Self { self.style.justify_items = justify_items; self.mark_dirty() }
    pub fn align_self(&mut self, align_self: AlignSelf) -> &mut Self { self.style.align_self = align_self; self.mark_dirty() }
    pub fn justify_self(&mut self, justify_self: JustifySelf) -> &mut Self { self.style.justify_self = justify_self; self.mark_dirty() }
    pub fn align_content(&mut self, align_content: AlignContent) -> &mut Self { self.style.align_content = align_content; self.mark_dirty() }
    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self { self.style.justify_content = justify_content; self.mark_dirty() }
    pub fn margin(&mut self, margin: UiRect) -> &mut Self { self.style.margin = margin; self.mark_dirty() }
    pub fn padding(&mut self, padding: UiRect) -> &mut Self { self.style.padding = padding; self.mark_dirty() }
    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Self { self.style.flex_direction = flex_direction; self.mark_dirty() }
    pub fn flex_wrap(&mut self, flex_wrap: FlexWrap) -> &mut Self { self.style.flex_wrap = flex_wrap; self.mark_dirty() }
    pub fn flex_grow(&mut self, flex_grow: f32) -> &mut Self { self.style.flex_grow = flex_grow; self.mark_dirty() }
    pub fn flex_shrink(&mut self, flex_shrink: f32) -> &mut Self { self.style.flex_shrink = flex_shrink; self.mark_dirty() }
    pub fn flex_basis(&mut self, flex_basis: Val) -> &mut Self { self.style.flex_basis = flex_basis; self.mark_dirty() }
    pub fn row_gap(&mut self, row_gap: Val) -> &mut Self { self.style.row_gap = row_gap; self.mark_dirty() }
    pub fn column_gap(&mut self, column_gap: Val) -> &mut Self { self.style.column_gap = column_gap; self.mark_dirty() }

    // text area ez functions
    pub fn default_text(&mut self, default: impl Into<String>) -> &mut Self {
        match &self.ui {
            UI::TextArea { ghost_text, selected_bg, selected_border, text_color, font_size, .. } => 
                self.ui = UI::TextArea { default_text: default.into(), ghost_text: ghost_text.clone(), selected_bg: *selected_bg, selected_border: *selected_border, text_color: *text_color, font_size: *font_size },
            _ => warn!("Attempted to get a text area from a non text area element!")
        }
        self.mark_dirty()
    }

    pub fn ghost_text(&mut self, ghost: impl Into<String>) -> &mut Self {
        match &self.ui {
            UI::TextArea { default_text, selected_bg, selected_border, text_color, font_size, .. } => 
                self.ui = UI::TextArea { default_text: default_text.clone(), ghost_text: ghost.into(), selected_bg: *selected_bg, selected_border: *selected_border, text_color: *text_color, font_size: *font_size },
            _ => warn!("Attempted to get a text area from a non text area element!")
        }
        self.mark_dirty()
    }

    pub fn selected_background(&mut self, background: Color) -> &mut Self {
        match &self.ui {
            UI::TextArea { default_text, ghost_text, selected_border, text_color, font_size, .. } => 
                self.ui = UI::TextArea { default_text: default_text.clone(), ghost_text: ghost_text.clone(), selected_bg: Some(background), selected_border: *selected_border, text_color: *text_color, font_size: *font_size },
            _ => warn!("Attempted to get a text area from a non text area element!")
        }
        self.mark_dirty()
    }

    pub fn selected_border(&mut self, border: Color) -> &mut Self {
        match &self.ui {
            UI::TextArea { default_text, ghost_text, selected_bg, text_color, font_size, .. } => 
                self.ui = UI::TextArea { default_text: default_text.clone(), ghost_text: ghost_text.clone(), selected_bg: *selected_bg, selected_border: Some(border), text_color: *text_color, font_size: *font_size },
            _ => warn!("Attempted to get a text area from a non text area element!")
        }
        self.mark_dirty()
    }

    // slider ez functions
    pub fn first_color(&mut self, new: Color) -> &mut Self {
        match &self.ui {
            UI::Slider { direction, first: _, second, amount, moveable } =>
                self.ui = UI::Slider { direction: *direction, first: new, second: *second, amount: *amount, moveable: *moveable },
            _ => warn!("Attempted to get a slider from a non slider element!")
        };
        self.mark_dirty()
    }

    pub fn second_color(&mut self, new: Color) -> &mut Self {
        match &self.ui {
            UI::Slider { direction, first, second: _, amount, moveable } =>
                self.ui = UI::Slider { direction: *direction, first: *first, second: new, amount: *amount, moveable: *moveable },
            _ => warn!("Attempted to get a slider from a non slider element!")
        };
        self.mark_dirty()
    }

    pub fn amount(&mut self, new: f32) -> &mut Self {
        match &self.ui {
            UI::Slider { direction, first, second, amount: _, moveable } =>
                self.ui = UI::Slider { direction: *direction, first: *first, second: *second, amount: new, moveable: *moveable },
            _ => warn!("Attempted to get a slider from a non slider element!")
        };
        self.mark_dirty()
    }

    pub fn moveable(&mut self, new: bool) -> &mut Self {
        match &self.ui {
            UI::Slider { direction, first, second, amount, moveable: _ } =>
                self.ui = UI::Slider { direction: *direction, first: *first, second: *second, amount: *amount, moveable: new },
            _ => warn!("Attempted to get a slider from a non slider element!")
        };
        self.mark_dirty()
    }
   
    // button ez functions
    pub fn hover_color(&mut self, color: Color, border: Option<Color>) -> &mut UINode {
        match self.ui {
            UI::Button { hover_bg: _, press_bg } => {
                self.ui = UI::Button { hover_bg: Some(HoverColor(color, border)), press_bg }
            }
            _ => panic!("Variant has no hover color option!")
        }
        self.mark_dirty()
    }

    pub fn press_color(&mut self, color: Color, border: Option<Color>) -> &mut UINode {
        match self.ui {
            UI::Button { hover_bg, press_bg: _ } => {
                self.ui = UI::Button { hover_bg, press_bg: Some(PressColor(color, border)) }
            }
            _ => panic!("Variant has no press color option!")
        }
        self.mark_dirty()
    }

    // functions to update ui
    pub fn ui(&mut self, ui: UI) -> &mut UINode { self.ui = ui; self.mark_dirty() }

    // other functions to make lives easier
    pub fn children<F>(&mut self, f: F) -> &mut UINode where F: Fn(&mut UINode) { f(self); self }

    pub fn add(&mut self, ui: UI) -> &mut UINode {
        // add child
        self.children.push(UINode { 
            id: None, data: None, ui, 
            style: Style::default(), 
            background_color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
            border: None, representation: None, image: None,
            children: Vec::new(), is_dirty: true,
        });
        self.is_dirty = true;
        return self.children.last_mut().unwrap(); // kinda clunky but necessary for memory safety rust reasons
    }

    pub fn get(&self, id: impl Into<String>) -> Option<&UINode> {
        let id: &String = &id.into();

        // if this id is too self, return self
        if self.id.is_some() && id == self.id.as_ref().unwrap() { return Some(self) }
        else {
            // otherwise, attempt to find id in children
            let mut iter = self.children.iter();
            while let Some(node) = iter.next() {
                let found = node.get(id);
                if found.is_some() { return found }
            }

            // default to none
            return None;
        }
    }

    pub fn get_mut(&mut self, id: impl Into<String>) -> Option<&mut UINode> {
        let id: &String = &id.into();

        // if this id is too self, return self
        if self.id.is_some() && id == self.id.as_ref().unwrap() { return Some(self) }
        else {
            // otherwise, attempt to find id in children
            let mut iter = self.children.iter_mut();
            while let Some(node) = iter.next() {
                let found = node.get_mut(id);
                if found.is_some() { return found }
            }

            // default to none
            return None;
        }
    }

    pub fn remove(&mut self, id: impl Into<String>) -> &UINode {
        let id: &String = &id.into();

        // only keep child nodes if they do not have the same id as given
        self.children.retain_mut(|a| {
            a.remove(id);
            a.id.as_ref().unwrap() != id
        });
        self.mark_dirty()
    }
}
