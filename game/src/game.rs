use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    vec,
};

use crate::{
    components::{
        conditional_parent::{AnimationCondition, ConditionalParent},
        follow::Follow,
        multi_conditional_parent::MultiConditionalParent,
        player_part::PlayerPart,
        player_state::PlayerState,
    },
    extractors::postprocess_buffer_extractor::PostprocessBufferExtractor,
    foreground::Foreground,
    player_state_definition::PlayerStateDefinition,
    systems::{
        animation_system::AnimationSystem, camera_system::CameraSystem,
        current_system::CurrentSystem, follow_system::FollowSystem, input_system::InputSystem,
        player_movement_system::PlayerMovementSystem,
        terrain_collision_system::TerrainCollisionSystem, terrain_system::TerrainSystem,
        update_focal_radius_system::UpdateFocalRadiusSystem,
        update_god_rays_system::UpdateGodRaysSystem,
    },
    terrain::{terrain::Terrain, water::Water},
};
use include_assets::{include_dir, NamedArchive};
use lumina_engine::{
    logic::{
        ecs::{
            component::{
                camera::Camera,
                collider::{Collider, ColliderShape},
                emitter::Emitter,
                force::{AppliedForce, Force, ForceEffect, ForceMode},
                material::Material,
                model::Model,
                movement::Movement,
                parent::Parent,
                transform::Transform,
            },
            entity::{entity::Entity, particle_entity::ParticleEntityType},
        },
        scene::world::World,
    },
    math::{vec2::Vec2, vec3::Vec3},
    render::{
        resource::shader::{
            parameter_schema::ParameterSchema, shader_configuration::ShaderConfiguration,
            shader_parameter_type::ShaderParameterType,
        },
        resource::{
            resource_manager::ResourceManager,
            resource_provider::ResourceProvider,
            texture::texture::{StaticColor, Texture},
        },
        uniform_buffer_source::UniformBufferSource,
    },
    shared::postprocess_config::PostprocessConfig,
    spawn_entity,
};
use winit::event_loop::EventLoop;

pub fn initialize(event_loop: EventLoop<()>) {
    lumina_engine::app::start(event_loop, |scene, resource_manager| {
        load_resources(resource_manager);
        init_world(scene.get_world_mut(), resource_manager);
        scene.register_system(Box::new(InputSystem));
        scene.register_system(Box::new(PlayerMovementSystem));
        scene.register_system(Box::new(CurrentSystem));
        scene.register_system(Box::new(TerrainSystem));
        scene.register_system(Box::new(FollowSystem));
        scene.register_system(Box::new(CameraSystem));
        scene.register_system(Box::new(AnimationSystem));
        scene.register_system(Box::new(TerrainCollisionSystem));
        scene.register_system(Box::new(UpdateFocalRadiusSystem));
        scene.register_system(Box::new(UpdateGodRaysSystem));
        scene.register_extractor(Box::new(PostprocessBufferExtractor));
    });
}

fn load_resources(resource_manager: &mut ResourceManager) {
    resource_manager.attach_archive(NamedArchive::load(include_dir!("assets")));
    resource_manager
        .load_shader(
            "background",
            ShaderConfiguration {
                fragment_shader_name: "background.frag".to_string(),
                vertex_shader_name: "background.vert".to_string(),
                tess_evaluation_shader_name: None,
                tess_control_shader_name: None,
                parameter_schema: ParameterSchema {
                    required_params: vec![
                        ("uModelMatrix".to_string(), ShaderParameterType::Mat4),
                        ("uFlipped".to_string(), ShaderParameterType::Bool),
                        ("uColor1".to_string(), ShaderParameterType::Vec3),
                        ("uColor2".to_string(), ShaderParameterType::Vec3),
                        ("uLayerIndex".to_string(), ShaderParameterType::Float),
                    ],
                },
            },
        )
        .expect("Failed to load background shader");
    /*let mut square_mesh = resource_manager.get_mesh("square");
    let mut bubble_mesh = square_mesh.clone();
    /*bubble.set_scale(Vec2::uniform(0.01));
    if let Some(texture) = resource_manager.load_static_texture("bubble.png") {
        bubble.set_texture(texture);
    }*/
    let seagrass_mesh = resource_manager.load_mesh_from_texture(&texture).unwrap();
    resource_manager.save_mesh("seagrass", seagrass);*/

    /*let mut fish = square.clone();
    if let Some(texture) = resource_manager.load_static_texture("fish.png") {
        fish.set_texture(texture);
    }
    fish.set_scale(Vec2::uniform(0.04));
    */
    //resource_manager.save_mesh("bubble", bubble_mesh);
    //resource_manager.save_mesh("fish", fish);
}

fn init_world(world: &mut World, resource_manager: &mut ResourceManager) {
    init_background(world, resource_manager); // TODO: fix this hack where background is initialized after other entities, causing it to render "on top of them"
    const WORLD_SEED: u32 = 696969;
    let terrain = Terrain::generate(world, 6969, resource_manager);
    world.insert_resource(Arc::new(Mutex::new(terrain)));
    let water = Water::create((WORLD_SEED ^ 0x5EAF00D).wrapping_mul(69696969));
    world.insert_resource(water);
    let shader = resource_manager.get_shader("model").clone();
    let foreground = Foreground::construct();
    world.insert_resource(UniformBufferSource::new(
        1,
        foreground.get_default_uniform_buffer(),
    ));
    world.insert_resource(foreground);
    let postprocess_shader = resource_manager
        .load_shader(
            "postprocess",
            ShaderConfiguration {
                vertex_shader_name: "postprocess.vert".to_string(),
                fragment_shader_name: "postprocess.frag".to_string(),
                tess_control_shader_name: None,
                tess_evaluation_shader_name: None,
                parameter_schema: ParameterSchema {
                    required_params: vec![
                        ("uFocalOffset".to_string(), ShaderParameterType::Vec2),
                        ("uAspectRatio".to_string(), ShaderParameterType::Float),
                        ("uNumLights".to_string(), ShaderParameterType::Int),
                        (
                            "uLightPositions".to_string(),
                            ShaderParameterType::Vec2Array,
                        ),
                    ],
                },
            },
        )
        .expect("Failed to load postprocess shader");

    world.insert_resource(PostprocessConfig {
        material: Material::new(Texture::None, postprocess_shader),
    });

    let model_scale = 0.15;
    let initial_position = Vec3::new(0.0, 0.25, 0.0);

    // Create dummy
    spawn_entity!(
        world,
        Transform {
            position: (0.75, 0.0, 0.0).into(),
            rotation: 0.0,
            scale: Vec2::uniform(0.5),
            is_flipped: false,
        },
        Material::new(
            Texture::StaticColor(StaticColor::new((0.5, 0.5, 0.5).into())),
            shader.clone(),
        ),
        Model::from(resource_manager.get_mesh("square")),
        Collider {
            shape: ColliderShape::Capsule2D {
                width: 0.5,
                height: 1.0,
            },
            offset: (0.0, 0.25).into(),
        },
        {
            let mut force_component = Force::new(1.0);
            force_component.apply_force(AppliedForce {
                id: "water_resistance".to_string(),
                effect: ForceEffect::Drag(world.expect_resource::<Water>().get_resistance()),
                mode: ForceMode::Continuous,
            });
            force_component
        },
        Movement::default()
    );
    let dummy = world.create_entity();
    world.add_component(
        dummy,
        Transform {
            position: (0.75, 0.0, 0.0).into(),
            rotation: 0.0,
            scale: Vec2::uniform(0.5),
            is_flipped: false,
        },
    );
    world.add_component(
        dummy,
        Material::new(
            Texture::StaticColor(StaticColor::new((0.5, 0.5, 0.5).into())),
            shader.clone(),
        ),
    );
    world.add_component::<Model>(dummy, resource_manager.get_mesh("square").clone().into());
    world.add_component(
        dummy,
        Collider {
            shape: ColliderShape::Capsule2D {
                width: 0.5,
                height: 1.0,
            },
            offset: (0.0, 0.25).into(),
        },
    );
    let mut force_component = Force::new(1.0);
    force_component.apply_force(AppliedForce {
        id: "water_resistance".to_string(),
        effect: ForceEffect::Drag(world.expect_resource::<Water>().get_resistance()),
        mode: ForceMode::Continuous,
    });
    world.add_component(dummy, force_component);
    world.add_component(dummy, Movement::default());

    // Create dummy 2

    let dummy = world.create_entity();
    world.add_component(
        dummy,
        Transform {
            position: (-0.75, 0.0, 0.0).into(),
            rotation: 0.0,
            scale: Vec2::uniform(0.5),
            is_flipped: false,
        },
    );
    world.add_component(
        dummy,
        Material::new(
            Texture::StaticColor(StaticColor::new((0.5, 0.5, 0.5).into())),
            shader.clone(),
        ),
    );
    world.add_component::<Model>(dummy, resource_manager.get_mesh("square").clone().into());
    world.add_component(
        dummy,
        Collider {
            shape: ColliderShape::Rect {
                width: 0.5,
                height: 0.5,
            },
            offset: (0.0, 0.0).into(),
        },
    );
    let mut force_component = Force::new(1.0);
    force_component.apply_force(AppliedForce {
        id: "water_resistance".to_string(),
        effect: ForceEffect::Drag(world.expect_resource::<Water>().get_resistance()),
        mode: ForceMode::Continuous,
    });
    world.add_component(dummy, force_component);
    world.add_component(dummy, Movement::default());

    // Create player

    let player = world.create_entity();
    let (camera, _) = world
        .query::<(&Camera,)>()
        .next()
        .expect("No camera found in the scene");

    world.add_component(
        camera,
        Follow {
            max_distance: 0.25,
            target_entity: player,
        },
    );

    /*world.add_component::<ColliderComponent>(
        player,
        Collider::rect(0.4, 1.4, (-0.05, -0.05).into()).into(),
    );*/
    world.add_component::<Collider>(
        player,
        Collider {
            shape: ColliderShape::Rect {
                width: 0.4,
                height: 1.4,
            },
            //offset: (-0.05, -0.05).into(),
            offset: (0.0, 0.0).into(),
        },
    );
    world.add_component(player, PlayerState::Idle);
    world.add_component(player, Movement::default());
    let mut force_component = Force::new(10.0);
    force_component.apply_force(AppliedForce {
        id: "water_resistance".to_string(),
        effect: ForceEffect::Drag(world.expect_resource::<Water>().get_resistance()),
        mode: ForceMode::Continuous,
    });
    world.add_component(player, force_component);
    world.add_component(
        player,
        Transform {
            position: initial_position,
            scale: Vec2::new(model_scale, model_scale),
            rotation: 0.0,
            is_flipped: false,
        },
    );

    let left_hand_model = world.create_entity();
    let legs_model = world.create_entity();
    let moving_legs_model = world.create_entity();
    let torso_model = world.create_entity();
    let right_hand_model = world.create_entity();
    let tank_model = world.create_entity();
    let head_model = world.create_entity();
    let moving_head_model = world.create_entity();

    let mut head_textures: Vec<&str> = vec!["player/head2.png", "player/head3.png"];

    let count = 22;
    for _ in 0..count {
        head_textures.push("player/head0.png");
        head_textures.push("player/head1.png");
    }

    let moving_legs_textures: &[&str] = &["player/legs0.png", "player/legs1.png"];
    let left_hand_texture = resource_manager
        .load_static_texture("player/left_hand.png")
        .unwrap();
    let legs_texture = resource_manager
        .load_static_texture("player/legs0.png")
        .unwrap();
    let torso_texture = resource_manager
        .load_static_texture("player/torso.png")
        .unwrap();
    let right_hand_texture = resource_manager
        .load_static_texture("player/right_hand.png")
        .unwrap();
    let tank_texture = resource_manager
        .load_static_texture("player/tank.png")
        .unwrap();
    let head_texture = resource_manager
        .load_animated_texture(head_textures.as_slice(), 6000)
        .unwrap();
    let moving_legs_texture = resource_manager
        .load_animated_texture(
            moving_legs_textures,
            PlayerStateDefinition::Swimming.legs_animation_time(),
        )
        .unwrap();
    let moving_head_texture = head_texture.clone();

    world.add_component(
        left_hand_model,
        Material::new(left_hand_texture.into(), shader.clone()),
    );
    world.add_component(
        legs_model,
        Material::new(legs_texture.into(), shader.clone()),
    );
    world.add_component(
        torso_model,
        Material::new(torso_texture.into(), shader.clone()),
    );
    world.add_component(
        right_hand_model,
        Material::new(right_hand_texture.into(), shader.clone()),
    );
    world.add_component(
        tank_model,
        Material::new(tank_texture.into(), shader.clone()),
    );
    world.add_component(
        head_model,
        Material::new(head_texture.into(), shader.clone()),
    );
    world.add_component(
        moving_legs_model,
        Material::new(moving_legs_texture.into(), shader.clone()),
    );
    world.add_component(
        moving_head_model,
        Material::new(moving_head_texture.into(), shader.clone()),
    );

    let initial_scales = vec![
        0.31640625, 0.4375, 0.23828125, 0.32421875, 0.25, 0.23828125, 0.4375, 0.23828125,
    ];

    let initial_positions = vec![
        Vec3::from_vec2(Vec2::new(0.08984375, -0.03515625) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.03125, -0.3984375) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.05078125, 0.09765625) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.03515625, -0.05078125) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.2265625, 0.125) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.03515625, 0.54296875) * model_scale, 0.0),
        Vec3::from_vec2(Vec2::new(-0.03125, -0.3984375) * model_scale, 0.0),
        Vec3::from_vec2(
            Vec2::new(-0.03515625 - 0.05, 0.54296875 - 0.075) * model_scale,
            0.0,
        ),
    ];

    let pattern_mesh = resource_manager.get_mesh("square");

    let children = vec![
        left_hand_model,
        legs_model,
        torso_model,
        right_hand_model,
        tank_model,
        head_model,
        moving_legs_model,
        moving_head_model,
    ];

    world.add_component(left_hand_model, PlayerPart::LeftHand);

    world.add_component(legs_model, PlayerPart::Legs);
    world.add_component(torso_model, PlayerPart::Torso);
    world.add_component(right_hand_model, PlayerPart::RightHand);
    world.add_component(tank_model, PlayerPart::Tank);
    world.add_component(head_model, PlayerPart::Head);
    world.add_component(moving_legs_model, PlayerPart::Legs);
    world.add_component(moving_head_model, PlayerPart::Head);

    for (idx, child) in children.iter().enumerate() {
        world.add_component(
            *child,
            Transform {
                position: initial_positions[idx],
                rotation: match idx {
                    7 => PI / 2.0,
                    _ => 0.0,
                },
                scale: Vec2::new(2.0, 2.0) * initial_scales[idx],
                is_flipped: false,
            },
        );
        world.add_component::<Model>(*child, pattern_mesh.clone().into());
        match idx {
            1 | 6 => {
                let condition = if idx == 1 {
                    AnimationCondition::PlayerIdle
                } else {
                    AnimationCondition::PlayerSwimming
                };
                world.add_component::<MultiConditionalParent>(
                    *child,
                    vec![
                        ConditionalParent {
                            parent: player,
                            condition: condition,
                        },
                        ConditionalParent {
                            parent: Entity(0).into(),
                            condition: AnimationCondition::True,
                        },
                    ]
                    .into(),
                );
                world.add_component::<Parent>(*child, Entity(0).into());
            }
            5 | 7 => {
                let condition = if idx == 5 {
                    AnimationCondition::PlayerIdle
                } else {
                    AnimationCondition::PlayerSwimming
                };
                world.add_component::<MultiConditionalParent>(
                    *child,
                    vec![
                        ConditionalParent {
                            parent: player,
                            condition: condition,
                        },
                        ConditionalParent {
                            parent: Entity(0).into(),
                            condition: AnimationCondition::True,
                        },
                    ]
                    .into(),
                );
                world.add_component::<Parent>(*child, Entity(0).into())
            }
            _ => world.add_component::<Parent>(*child, player.into()),
        }
    }

    let bubble_mesh = resource_manager.get_mesh("square");
    let bubble_emitter = world.create_entity();
    world.add_component::<Emitter>(bubble_emitter, ParticleEntityType::Bubble.into());
    world.add_component::<Transform>(
        bubble_emitter,
        Transform {
            position: (0.025, -0.025, 0.0001).into(),
            rotation: 0.0,
            scale: Vec2::uniform(0.01),
            is_flipped: false,
        },
    );
    // TODO: use prefab for bubble emitter
    world.add_component::<Model>(bubble_emitter, bubble_mesh.into());
    if let Some(texture) = resource_manager.load_static_texture("bubble.png") {
        world.add_component(
            bubble_emitter,
            Material::new(texture.into(), shader.clone()),
        );
    }

    world.add_component::<MultiConditionalParent>(
        bubble_emitter,
        vec![
            ConditionalParent {
                parent: moving_head_model,
                condition: AnimationCondition::PlayerSwimming,
            },
            ConditionalParent {
                parent: head_model,
                condition: AnimationCondition::PlayerIdle,
            },
        ]
        .into(),
    );
    world.add_component::<Parent>(bubble_emitter, head_model.into());
}

fn init_background(world: &mut World, resource_manager: &mut ResourceManager) {
    let background = world.create_entity();
    world.add_component(
        background,
        Transform {
            position: (0.0, 0.0, -7.5).into(),
            rotation: 0.0,
            scale: Vec2::uniform(2.0),
            is_flipped: false,
        },
    );
    world.add_component(
        background,
        Material::new(
            Texture::None,
            resource_manager.get_shader("background").clone(),
        )
        .with_param("uColor1", Vec3::new(0.0, 0.29, 0.43))
        .with_param("uColor2", Vec3::new(0.0, 0.5, 0.5)),
    );
    let pattern_mesh = resource_manager.get_mesh("square");
    world.add_component::<Model>(background, pattern_mesh.clone().into());
}
