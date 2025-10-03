#[cfg(test)]
mod world_test {
    use include_assets::NamedArchive;
    use lumina::engine::model::model::Model;
    use lumina::engine::model::model_group::ModelGroup;
    use lumina::engine::scene::world::component::component::Component;
    use lumina::engine::scene::world::component::model_component::ModelComponent;
    use lumina::engine::scene::world::entity::entity::Entity;
    use lumina::engine::scene::world::world::World;
    use lumina::engine::texture::font_texture::FontTexture;
    use lumina::engine::texture::resource_provider::ResourceProvider;
    use lumina::engine::texture::texture::Texture;
    use lumina::engine::texture::texture_handler::TextureHandler;
    use mockall::mock;

    mock! {
        pub ResourceProvider {}
        impl ResourceProvider for ResourceProvider {
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
            fn load_seagrass<'a>(
                &mut self,
                texture_names: &[&'a str],
            ) -> ModelGroup;
        }
    }

    mock! {
        pub Component {}
        impl Component for Component {}
    }

    fn create_world() -> World {
        World::load()
    }

    #[test]
    fn test_world_creation() {
        let world = create_world();
        let null_entity_opt = world.entities.get(0);
        assert!(null_entity_opt.is_some());
        assert_eq!(Entity(0), *null_entity_opt.unwrap());
        assert_eq!(0, world.components.len());
        assert_eq!(0, world.particle_pool.len());
        assert_eq!(0, world.renderables.len());
    }

    #[test]
    fn test_create_entity() {
        let mut world = create_world();
        assert_eq!(Entity(1), world.create_entity());
        assert_eq!(2, world.entities.len());
    }

    #[test]
    fn test_add_component() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        assert_eq!(1, world.components.len())
    }

    #[test]
    fn test_get_component() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        assert!(world.get_component::<MockComponent>(entity).is_some());
        assert!(world.get_component_mut::<MockComponent>(entity).is_some());

        assert!(world.get_component::<ModelComponent>(entity).is_none());
        assert!(world.get_component_mut::<ModelComponent>(entity).is_none());
    }

    #[test]
    fn test_delete_entity() {
        let mut world = create_world();

        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        world.delete_entity(entity);

        assert_eq!(1, world.entities.len());
        assert!(world.get_component::<MockComponent>(entity).is_none());
        assert!(world.get_component_mut::<MockComponent>(entity).is_none());

        let new_entity = world.create_entity();
        assert_eq!(Entity(1), new_entity);
    }

    #[test]
    fn test_get_storage() {
        let mut world = create_world();
        let entity = world.create_entity();
        assert!(world.get_storage::<MockComponent>().is_none());
        assert!(world.get_storage_ptr::<MockComponent>().is_none());
        world.add_component::<MockComponent>(entity, MockComponent {});
        assert!(world.get_storage::<MockComponent>().is_some());
        assert!(world.get_storage_ptr::<MockComponent>().is_some());
    }
    #[test]
    fn test_insert_resource() {
        let mut world = create_world();
        assert!(world.get_resource::<MockComponent>().is_none());
        assert!(world.get_resource_mut::<MockComponent>().is_none());
        world.insert_resource(MockComponent {});
        assert!(world.get_resource::<MockComponent>().is_some());
        assert!(world.get_resource_mut::<MockComponent>().is_some());
    }
}
