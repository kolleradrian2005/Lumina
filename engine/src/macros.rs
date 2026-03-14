#[macro_export]
macro_rules! spawn_entity {
    ($world:expr, $($component:expr),* $(,)?) => {{
        let entity = $world.create_entity();
        $(
            $world.add_component(entity, $component);
        )*
        entity
    }};
}
