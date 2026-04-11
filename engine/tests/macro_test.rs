use lumina_engine::{
    logic::{ecs::component::component::Component, scene::world::World},
    spawn_entity,
};

#[derive(Component)]
struct TestComponent {
    value: i32,
}

#[derive(Component)]
struct Tag;

#[derive(Component)]
struct Counter(u32);

fn assert_component<T: Component>() {}

#[test]
fn component_derive_implements_component_trait() {
    assert_component::<TestComponent>();
    assert_component::<Tag>();
    assert_component::<Counter>();
}

#[test]
fn component_derive_allows_world_storage_round_trip() {
    let mut world = World::load();
    let entity = world.create_entity();

    world.add_component(entity, TestComponent { value: 42 });

    let component = world.get_component::<TestComponent>(entity).unwrap();
    assert_eq!(component.value, 42);
}

#[test]
fn spawn_entity_adds_every_component() {
    let mut world = World::load();

    let entity = spawn_entity!(world, TestComponent { value: 7 }, Tag, Counter(3),);

    assert!(world.entities.contains(&entity));
    assert_eq!(
        world.get_component::<TestComponent>(entity).unwrap().value,
        7
    );
    assert!(world.get_component::<Tag>(entity).is_some());
    assert_eq!(world.get_component::<Counter>(entity).unwrap().0, 3);
}
