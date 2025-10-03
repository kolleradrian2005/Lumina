use std::{
    ops::Deref,
    sync::{Arc, Mutex, PoisonError},
};

use include_assets::NamedArchive;

use crate::engine::model::{model::Model, model_group::ModelGroup};

use super::{font_texture::FontTexture, texture::Texture, texture_handler::TextureHandler};

#[derive(Clone)]
pub struct ResourceProviderHandle {
    pub inner: Arc<Mutex<dyn ResourceProvider>>,
}

impl ResourceProviderHandle {
    pub fn new(provider: impl ResourceProvider + 'static) -> Self {
        Self {
            inner: Arc::new(Mutex::new(provider)),
        }
    }
    pub fn get_inner(&self) -> Arc<Mutex<dyn ResourceProvider>> {
        self.inner.clone()
    }
    pub fn lock(
        &self,
    ) -> Result<
        std::sync::MutexGuard<'_, (dyn ResourceProvider + 'static)>,
        PoisonError<std::sync::MutexGuard<'_, (dyn ResourceProvider + 'static)>>,
    > {
        self.inner.lock()
    }
}

impl Deref for ResourceProviderHandle {
    type Target = Mutex<dyn ResourceProvider>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait ResourceProvider: Send + Sync {
    fn get_archive(&self) -> &NamedArchive;
    fn preload_models(&mut self);
    fn load_fonts(&mut self);
    fn save_model(&mut self, name: &str, model: Model);
    fn save_model_group(&mut self, name: &str, model_group: ModelGroup);
    fn get_model(&self, name: &str) -> Model;
    fn get_model_group(&self, name: &str) -> ModelGroup;
    fn save_font(&mut self, name: &str, font: FontTexture);
    fn get_font(&self, name: &str) -> FontTexture;
    fn get_texture_handler_mut(&mut self) -> &mut TextureHandler;
    fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture>;
    fn load_animated_texture<'a>(
        &mut self,
        texture_names: &[&'a str],
        animation_time: u128,
    ) -> Option<Texture>;
    fn load_seagrass<'a>(&mut self, texture_names: &[&'a str]) -> ModelGroup;
}
