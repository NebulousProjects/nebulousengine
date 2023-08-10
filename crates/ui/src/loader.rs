use bevy::{prelude::*, asset::{LoadedAsset, AssetLoader}};

use crate::serializables::UiElement;

// asset loader to load ui files
pub struct UiLoader;
impl AssetLoader for UiLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // load content
            let content = std::str::from_utf8(bytes);
            if content.is_err() { error!("Failed to load ui json!"); return Err(bevy::asset::Error::msg("Failed to load json")) }
            let content = content.unwrap();
            
            // load description
            let description: Result<UiElement, serde_json::Error> = serde_json::from_str(content);
            if description.is_err() { error!("Failed to load ui description from json with error: {}", description.err().unwrap()); return Err(bevy::asset::Error::msg("Failed to load description")) }
            
            // load final input map
            load_context.set_default_asset(LoadedAsset::new(description.unwrap()));
            
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ui"]
    }
}