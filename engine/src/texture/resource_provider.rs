use std::{
    ops::Deref,
    sync::{Arc, Mutex, PoisonError},
};

use include_assets::NamedArchive;

use crate::{
    model::model::Model,
    shader::{shader_configuration::ShaderConfiguration, shader_program::ShaderProgram},
};

use super::texture::Texture;

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
        std::sync::MutexGuard<'_, dyn ResourceProvider + 'static>,
        PoisonError<std::sync::MutexGuard<'_, dyn ResourceProvider + 'static>>,
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
    fn load_default_models(&mut self);
    fn save_model(&mut self, name: &str, model: Model);
    fn get_model(&self, name: &str) -> Model;
    fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture>;
    fn load_animated_texture<'a>(
        &mut self,
        texture_names: &[&'a str],
        animation_time: u128,
    ) -> Option<Texture>;
    fn attach_archive(&mut self, archive: NamedArchive);
    fn get_shader(&self, shader_name: &str) -> Arc<ShaderProgram>;
    fn load_shader(
        &mut self,
        shader_name: &str,
        shader_configuration: ShaderConfiguration,
    ) -> Option<Arc<ShaderProgram>>;
    //fn load_fonts(&mut self);
    //fn get_font(&self, name: &str) -> FontTexture;
}
