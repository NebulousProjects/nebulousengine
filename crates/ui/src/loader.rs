use bevy::{prelude::*, asset::{AssetLoader, AsyncReadExt}};

use crate::serializables::UiElement;

#[derive(Debug)]
pub struct UiElementLoadError(String);

impl std::fmt::Display for UiElementLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiElementLoadError({})", self.0)
    }
}

impl std::error::Error for UiElementLoadError {}

// asset loader to load ui files
#[derive(Default)]
pub struct UiLoader;
impl AssetLoader for UiLoader {
    type Asset = UiElement;
    type Error = UiElementLoadError;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // load content
            let mut bytes = Vec::new();
            let error = reader.read_to_end(&mut bytes).await;
            if error.is_err() { return Err(UiElementLoadError("Failed to load text bytes!".into())) }
            let content = std::str::from_utf8(&bytes);
            if content.is_err() { error!("Failed to load ui json!"); return Err(UiElementLoadError("Failed to load json".into())) }
            let content = content.unwrap();
            
            // load description
            let description: Result<UiElement, serde_json::Error> = serde_json::from_str(content);
            if description.is_err() { error!("Failed to load ui description from json with error: {}", description.err().unwrap()); return Err(UiElementLoadError("Failed to load description".into())) }
            
            // load final input map
            // load_context.set_default_asset(LoadedAsset::new());
            
            Ok(description.unwrap())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ui"]
    }
}