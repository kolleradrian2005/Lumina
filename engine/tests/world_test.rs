#[cfg(test)]
mod world_test {
    use lumina_engine::logic::ecs::component::component::Component;
    use lumina_engine::logic::ecs::component::model::Model;
    use lumina_engine::logic::ecs::component::movement::Movement;
    use lumina_engine::logic::ecs::component::transform::Transform;
    use lumina_engine::logic::ecs::entity::entity::Entity;
    use lumina_engine::logic::scene::world::World;
    use lumina_engine::math::vec2::Vec2;
    use lumina_engine::math::vec3::Vec3;
    use lumina_engine::render::mesh::Mesh;
    use lumina_engine::render::resource::resource_provider::ResourceProvider;
    use lumina_engine::render::resource::shader::shader_configuration::ShaderConfiguration;
    use lumina_engine::render::resource::shader::shader_program::ShaderProgram;
    use lumina_engine::render::resource::texture::texture::Texture;
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        pub ResourceProvider {}
        impl ResourceProvider for ResourceProvider {
            fn load_default_meshes(&mut self);
            fn load_default_shaders(&mut self);
            fn save_mesh(&mut self, name: &str, mesh: Mesh);
            fn get_mesh(&self, name: &str) -> Arc<Mesh>;
            fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture>;
            fn load_animated_texture<'a>(
                &mut self,
                texture_names: &[&'a str],
                animation_time: u128,
            ) -> Option<Texture>;
            fn attach_archive(&mut self, archive: include_assets::NamedArchive);
            fn get_shader(&self, shader_name: &str) -> Arc<ShaderProgram>;
            fn load_shader(
                &mut self,
                shader_name: &str,
                shader_configuration: ShaderConfiguration,
            ) -> Option<Arc<ShaderProgram>>;
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
    }

    #[test]
    fn test_create_entity() {
        let mut world = create_world();
        assert_eq!(Entity(1), world.create_entity());
        assert_eq!(2, world.entities.len());
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

        assert!(world.get_component::<Model>(entity).is_none());
        assert!(world.get_component_mut::<Model>(entity).is_none());
    }

    #[test]
    fn test_remove_component() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        assert!(world.get_component::<MockComponent>(entity).is_some());
        assert!(world.get_component_mut::<MockComponent>(entity).is_some());

        let removed = world.remove_component::<MockComponent>(entity);
        assert!(removed.is_some());

        assert!(world.get_component::<MockComponent>(entity).is_none());
        assert!(world.get_component_mut::<MockComponent>(entity).is_none());
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

    #[test]
    fn test_create_multiple_entities() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        let e3 = world.create_entity();
        assert_eq!(e1, Entity(1));
        assert_eq!(e2, Entity(2));
        assert_eq!(e3, Entity(3));
        assert_eq!(world.entities.len(), 4); // null entity + 3
    }

    #[test]
    fn test_delete_entity_reuses_id_lifo() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        world.delete_entity(e1);
        world.delete_entity(e2);
        let e3 = world.create_entity();
        assert_eq!(e3, Entity(2));
        let e4 = world.create_entity();
        assert_eq!(e4, Entity(1));
    }

    #[test]
    fn test_delete_nonexistent_entity() {
        let mut world = create_world();
        // Should not panic
        world.delete_entity(Entity(999));
        assert_eq!(world.entities.len(), 1);
    }

    #[test]
    fn test_delete_entity_removes_all_components() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        world.delete_entity(entity);
        // Component storage should exist but entity entry should be gone
        assert!(world.get_component::<MockComponent>(entity).is_none());
    }

    #[test]
    fn test_add_component_to_multiple_entities() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        world.add_component::<MockComponent>(e1, MockComponent {});
        world.add_component::<MockComponent>(e2, MockComponent {});
        assert!(world.get_component::<MockComponent>(e1).is_some());
        assert!(world.get_component::<MockComponent>(e2).is_some());
        assert_eq!(world.components.len(), 1); // same type, one storage
    }

    #[test]
    fn test_overwrite_component() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        world.add_component::<MockComponent>(entity, MockComponent {});
        // Should still have the component (overwritten)
        assert!(world.get_component::<MockComponent>(entity).is_some());
    }

    #[test]
    fn test_remove_component_returns_none_when_missing() {
        let mut world = create_world();
        let entity = world.create_entity();
        let removed = world.remove_component::<MockComponent>(entity);
        assert!(removed.is_none());
    }

    #[test]
    fn test_get_component_wrong_entity() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        world.add_component::<MockComponent>(e1, MockComponent {});
        assert!(world.get_component::<MockComponent>(e2).is_none());
    }

    #[test]
    fn test_get_component_mut_modifies() {
        let mut world = create_world();
        world.insert_resource(42u32);
        // Verify we can get and modify a resource via mutable access
        {
            let r = world.get_resource_mut::<u32>().unwrap();
            *r = 100;
        }
        assert_eq!(*world.get_resource::<u32>().unwrap(), 100);
    }

    #[test]
    fn test_insert_resource_overwrites() {
        let mut world = create_world();
        world.insert_resource(42u32);
        world.insert_resource(99u32);
        assert_eq!(*world.get_resource::<u32>().unwrap(), 99);
    }

    #[test]
    fn test_expect_resource() {
        let mut world = create_world();
        world.insert_resource(String::from("hello"));
        let r = world.expect_resource::<String>();
        assert_eq!(r, "hello");
    }

    #[test]
    #[should_panic(expected = "not found")]
    fn test_expect_resource_panics_when_missing() {
        let world = create_world();
        world.expect_resource::<String>();
    }

    #[test]
    fn test_expect_resource_mut() {
        let mut world = create_world();
        world.insert_resource(42u32);
        let r = world.expect_resource_mut::<u32>();
        *r = 100;
        assert_eq!(*world.expect_resource::<u32>(), 100);
    }

    #[test]
    #[should_panic(expected = "not found")]
    fn test_expect_resource_mut_panics_when_missing() {
        let mut world = create_world();
        world.expect_resource_mut::<String>();
    }

    #[test]
    fn test_get_resource_ptr() {
        let mut world = create_world();
        world.insert_resource(42u32);
        let ptr = world.get_resource_ptr::<u32>();
        assert!(ptr.is_some());
    }

    #[test]
    fn test_get_resource_ptr_none() {
        let mut world = create_world();
        let ptr = world.get_resource_ptr::<u32>();
        assert!(ptr.is_none());
    }

    #[test]
    #[should_panic(expected = "not found")]
    fn test_expect_resource_ptr_panics_when_missing() {
        let mut world = create_world();
        world.expect_resource_ptr::<String>();
    }

    #[test]
    fn test_multiple_resource_types() {
        let mut world = create_world();
        world.insert_resource(42u32);
        world.insert_resource(String::from("hello"));
        world.insert_resource(3.14f64);
        assert_eq!(*world.get_resource::<u32>().unwrap(), 42);
        assert_eq!(world.get_resource::<String>().unwrap(), "hello");
        assert_eq!(*world.get_resource::<f64>().unwrap(), 3.14);
    }

    #[test]
    fn test_get_storage_ptr_mut() {
        let mut world = create_world();
        let entity = world.create_entity();
        world.add_component::<MockComponent>(entity, MockComponent {});
        assert!(world.get_storage_ptr_mut::<MockComponent>().is_some());
    }

    #[test]
    fn test_get_storage_ptr_mut_none() {
        let mut world = create_world();
        assert!(world.get_storage_ptr_mut::<MockComponent>().is_none());
    }

    #[test]
    fn test_null_entity_is_preserved() {
        let mut world = create_world();
        // Create and delete entities, null entity (0) should remain
        let e1 = world.create_entity();
        world.delete_entity(e1);
        assert!(world.entities.contains(&Entity(0)));
    }

    #[test]
    fn test_query_single_component() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        world.add_component(
            e1,
            Transform {
                position: Vec3::new(1.0, 0.0, 0.0),
                rotation: 0.0,
                scale: Vec2::unit(),
                is_flipped: false,
            },
        );
        world.add_component(
            e2,
            Transform {
                position: Vec3::new(2.0, 0.0, 0.0),
                rotation: 0.0,
                scale: Vec2::unit(),
                is_flipped: false,
            },
        );

        let results: Vec<_> = world.query::<(&Transform,)>().collect();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_filters_entities_without_component() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let _e2 = world.create_entity(); // no transform
        world.add_component(e1, MockComponent {});

        let results: Vec<_> = world.query::<(&MockComponent,)>().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, e1);
    }

    #[test]
    fn test_query_empty_world() {
        let world = create_world();
        let results: Vec<_> = world.query::<(&MockComponent,)>().collect();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_query_mut_single_component() {
        let mut world = create_world();
        let e1 = world.create_entity();
        world.add_component(
            e1,
            Transform {
                position: Vec3::new(1.0, 0.0, 0.0),
                rotation: 0.0,
                scale: Vec2::unit(),
                is_flipped: false,
            },
        );

        for (_entity, (transform,)) in world.query_mut::<(&mut Transform,)>() {
            transform.position.x = 99.0;
        }

        let t = world.get_component::<Transform>(e1).unwrap();
        assert_eq!(t.position.x, 99.0);
    }

    #[test]
    fn test_query_double_component() {
        let mut world = create_world();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        world.add_component(e1, MockComponent {});
        world.add_component(e1, Movement::default());
        // e2 only has Transform, not Movement
        world.add_component(e2, MockComponent {});

        let results: Vec<_> = world.query::<(&MockComponent, &Movement)>().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, e1);
    }
}
