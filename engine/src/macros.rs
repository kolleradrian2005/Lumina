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

macro_rules! gl_check_error {
    () => {
        crate::render::gl_error::check_gl_error(file!(), line!())
    };
}
