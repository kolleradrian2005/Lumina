use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use crate::{
    components::{
        conditional_parent_component::{AnimationCondition, ConditionalParentComponent},
        follow_component::FollowComponent,
        multi_conditional_parent_component::MultiConditionalParentComponent,
        player_part_component::PlayerPartComponent,
        player_state_component::PlayerStateComponent,
    },
    player_state::PlayerState,
    systems::{
        animation_system::AnimationSystem, camera_system::CameraSystem,
        current_system::CurrentSystem, follow_system::FollowSystem, input_system::InputSystem,
        player_movement_system::PlayerMovementSystem,
    },
};
use include_assets::{include_dir, NamedArchive};
use lumina_engine::{
    collider::Collider,
    math::{vec2::Vec2, vec3::Vec3},
    model::model::Model,
    scene::{
        terrain::Terrain,
        world::{
            component::{
                camera_component::CameraComponent, collider_component::ColliderComponent,
                emitter_component::EmitterComponent, model_component::ModelComponent,
                movement_component::MovementComponent, parent_component::ParentComponent,
                texture_component::TextureComponent, transform_component::TransformComponent,
            },
            entity::{entity::Entity, particle_entity::ParticleEntityType},
            world::World,
        },
    },
    texture::{
        resource_manager::ResourceManager,
        resource_provider::ResourceProvider,
        texture::{StaticColor, Texture},
    },
    transformable::Transformable,
};
use winit::event_loop::EventLoop;

pub fn initialize(event_loop: EventLoop<()>) {
    // TODO: Move scene out of render context?
    lumina_engine::start(event_loop, |scene, resource_manager| {
        load_resources(resource_manager);
        //let mut scene = render_ctx.scene.lock().expect("Failed to lock scene");
        init_world(scene.get_world_mut(), resource_manager);
        scene.register_system(Box::new(InputSystem));
        scene.register_system(Box::new(PlayerMovementSystem));
        scene.register_system(Box::new(CurrentSystem));
        //scene.register_system(Box::new(TerrainSystem));
        scene.register_system(Box::new(FollowSystem));
        scene.register_system(Box::new(CameraSystem));
        scene.register_system(Box::new(AnimationSystem));
        //scene.register_system(Box::new(UpdateFocalRadiusSystem));
        //scene.register_system(Box::new(UpdateGodRaysSystem));
    });
}

fn load_resources(resource_manager: &mut ResourceManager) {
    resource_manager.attach_archive(NamedArchive::load(include_dir!("assets")));
    let mut square = resource_manager.get_model("square");
    square.set_texture(StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into());
    let mut bubble = square.clone();
    bubble.set_scale(Vec2::uniform(0.01));
    if let Some(texture) = resource_manager.load_static_texture("bubble.png") {
        bubble.set_texture(texture);
    }
    if let Some(Texture::StaticTexture(texture)) =
        resource_manager.load_static_texture("seagrass0.png")
    {
        let seagrass_mesh = resource_manager.load_mesh_from_texture(&texture).unwrap();
        let mut seagrass = Model::new(seagrass_mesh);
        seagrass.set_texture(texture.into());
        seagrass.set_scale(Vec2::uniform(0.08));
        resource_manager.save_model("seagrass", seagrass);
    }

    let mut fish = square.clone();
    if let Some(texture) = resource_manager.load_static_texture("fish.png") {
        fish.set_texture(texture);
    }
    fish.set_scale(Vec2::uniform(0.04));

    resource_manager.save_model("bubble", bubble);
    resource_manager.save_model("fish", fish);
}

fn init_world(world: &mut World, resource_manager: &mut ResourceManager) {
    let terrain = Terrain::generate(world, 6969, resource_manager);
    world.insert_resource(Arc::new(Mutex::new(terrain)));

    let model_scale = 0.15;
    let initial_position = Vec3::new(0.0, 0.25, 0.0);

    let player = world.create_entity();
    let (camera, _) = world
        .query::<(&CameraComponent,)>()
        .next()
        .expect("No camera found in the scene");

    world.add_component(
        camera,
        FollowComponent {
            max_distance: 0.25,
            target_entity: player,
        },
    );

    world.add_component::<ColliderComponent>(
        player,
        Collider::rect(0.4, 1.4, (-0.05, -0.05).into()).into(),
    );
    world.add_component(player, PlayerStateComponent::Idle);
    world.add_component(player, MovementComponent::default());
    world.add_component(
        player,
        TransformComponent {
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
            PlayerState::Swimming.legs_animation_time(),
        )
        .unwrap();
    let moving_head_texture = head_texture.clone();

    world.add_component::<TextureComponent>(left_hand_model, left_hand_texture.into());
    world.add_component::<TextureComponent>(legs_model, legs_texture.into());
    world.add_component::<TextureComponent>(torso_model, torso_texture.into());
    world.add_component::<TextureComponent>(right_hand_model, right_hand_texture.into());
    world.add_component::<TextureComponent>(tank_model, tank_texture.into());
    world.add_component::<TextureComponent>(head_model, head_texture.into());
    world.add_component::<TextureComponent>(moving_legs_model, moving_legs_texture.into());
    world.add_component::<TextureComponent>(moving_head_model, moving_head_texture.into());

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

    let pattern_model = resource_manager.get_model("square");

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

    world.add_component(left_hand_model, PlayerPartComponent::LeftHand);
    world.add_component(legs_model, PlayerPartComponent::Legs);
    world.add_component(torso_model, PlayerPartComponent::Torso);
    world.add_component(right_hand_model, PlayerPartComponent::RightHand);
    world.add_component(tank_model, PlayerPartComponent::Tank);
    world.add_component(head_model, PlayerPartComponent::Head);
    world.add_component(moving_legs_model, PlayerPartComponent::Legs);
    world.add_component(moving_head_model, PlayerPartComponent::Head);

    for (idx, child) in children.iter().enumerate() {
        world.add_component(
            *child,
            TransformComponent {
                position: initial_positions[idx],
                rotation: match idx {
                    7 => PI / 2.0,
                    _ => 0.0,
                },
                scale: Vec2::new(2.0, 2.0) * initial_scales[idx],
                is_flipped: false,
            },
        );
        world.add_component::<ModelComponent>(*child, pattern_model.get_mesh().clone().into());
        match idx {
            1 | 6 => {
                let condition = if idx == 1 {
                    AnimationCondition::PlayerIdle
                } else {
                    AnimationCondition::PlayerSwimming
                };
                world.add_component::<MultiConditionalParentComponent>(
                    *child,
                    vec![
                        ConditionalParentComponent {
                            parent: player,
                            condition: condition,
                        },
                        ConditionalParentComponent {
                            parent: Entity(0).into(),
                            condition: AnimationCondition::True,
                        },
                    ]
                    .into(),
                );
                world.add_component::<ParentComponent>(*child, Entity(0).into());
            }
            5 | 7 => {
                let condition = if idx == 5 {
                    AnimationCondition::PlayerIdle
                } else {
                    AnimationCondition::PlayerSwimming
                };
                world.add_component::<MultiConditionalParentComponent>(
                    *child,
                    vec![
                        ConditionalParentComponent {
                            parent: player,
                            condition: condition,
                        },
                        ConditionalParentComponent {
                            parent: Entity(0).into(),
                            condition: AnimationCondition::True,
                        },
                    ]
                    .into(),
                );
                world.add_component::<ParentComponent>(*child, Entity(0).into())
            }
            _ => world.add_component::<ParentComponent>(*child, player.into()),
        }
    }

    let bubble_model = resource_manager.get_model("bubble");
    let bubble_emitter = world.create_entity();
    world.add_component::<EmitterComponent>(bubble_emitter, ParticleEntityType::Bubble.into());
    world.add_component::<TransformComponent>(
        bubble_emitter,
        TransformComponent {
            position: (0.025, -0.025, 0.0001).into(),
            rotation: bubble_model.get_rotation(),
            scale: bubble_model.get_scale(),
            is_flipped: false,
        },
    );
    world.add_component::<ModelComponent>(bubble_emitter, bubble_model.get_mesh().clone().into());
    world.add_component::<TextureComponent>(
        bubble_emitter,
        bubble_model.get_texture().clone().into(),
    );

    world.add_component::<MultiConditionalParentComponent>(
        bubble_emitter,
        vec![
            ConditionalParentComponent {
                parent: moving_head_model,
                condition: AnimationCondition::PlayerSwimming,
            },
            ConditionalParentComponent {
                parent: head_model,
                condition: AnimationCondition::PlayerIdle,
            },
        ]
        .into(),
    );
    world.add_component::<ParentComponent>(bubble_emitter, head_model.into());
}
