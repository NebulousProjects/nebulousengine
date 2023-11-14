use bevy::prelude::*;
use serde_json::Value;

#[derive(Resource, Default, Debug, Clone)]
pub struct UINode {
    pub id: Option<String>,
    pub data: Option<Value>,
    pub ui: UI,
    pub representation: Option<Entity>,
    pub children: Vec<UINode>,
    pub is_dirty: bool
}

impl UINode {
    pub fn id(&mut self, id: impl Into<String>) { self.id = Some(id.into()); self.is_dirty = true; }
    pub fn data(&mut self, data: Value) { self.data = Some(data); self.is_dirty = true; }

    pub fn add(&mut self, ui: UI) -> &mut UINode {
        self.children.push(UINode { id: None, data: None, ui, representation: None, children: Vec::new(), is_dirty: false });
        self.is_dirty = true;
        return self.children.last_mut().unwrap(); // kinda clunky but necessary for memory safety rust reasons
    }

    pub fn get(&self, id: &String) -> Option<&UINode> {
        // if this id is too self, return self
        if id == self.id.as_ref().unwrap() { return Some(self) }
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

    pub fn get_mut(&mut self, id: &String) -> Option<&mut UINode> {
        // if this id is too self, return self
        if id == self.id.as_ref().unwrap() { return Some(self) }
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

    pub fn remove(&mut self, id: &String) {
        // only keep child nodes if they do not have the same id as given
        self.children.retain_mut(|a| {
            a.remove(id);
            a.id.as_ref().unwrap() != id
        });
        self.is_dirty = true;
    }
}

#[derive(Resource, Debug, Clone)]
pub enum UI {
    Panel { style: Style },
    Text {
        style: Style,
        text: String
    },
    Button {
        style: Style,
        text: String,
        hover_bg: Option<Color>,
        press_bg: Option<Color>
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::Panel { style: Style::default() }
    }
}

// plugin for uis
pub struct ConfigurableUIPlugin;
impl Plugin for ConfigurableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UINode>()
            .add_systems(Update, update_ui);
    }
}

fn update_ui(
    ui: ResMut<UINode>
) {
    recr_render(ui.as_ref(), false);
}

fn recr_render(ui: &UINode, force_render: bool) {
    if ui.is_dirty || force_render {

    }
}
