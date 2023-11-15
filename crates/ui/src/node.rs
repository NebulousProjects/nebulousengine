use bevy::prelude::*;
use serde_json::*;

use crate::ui::UI;

#[derive(Resource, Default, Debug, Clone)]
pub struct UINode {
    pub id: Option<String>,
    pub data: Option<Value>,

    pub ui: UI,
    pub style: Style,
    pub background_color: Color,

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

    // enum ez functions
    pub fn panel(&mut self) -> &mut UINode { self.add(UI::Panel) }
    pub fn scroll_panel(&mut self, flex_direction: FlexDirection) -> &mut UINode { self.add(UI::ScrollPanel { flex_direction }) }
    pub fn text(&mut self, text: impl Into<String>) -> &mut UINode { self.add(UI::Text { text: text.into() }) }
    pub fn button(&mut self, hover_bg: Option<Color>, press_bg: Option<Color>) -> &mut UINode { self.add(UI::Button { hover_bg, press_bg }) }

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
            representation: None, 
            children: Vec::new(), is_dirty: true
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
