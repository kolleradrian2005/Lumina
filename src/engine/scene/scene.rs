use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use crate::engine::collider::Collider;

use crate::engine::math::vec2::Vec2;
use crate::engine::math::vec3::Vec3;

use crate::engine::scene::player_state::PlayerState;
use crate::engine::scene::terrain::Terrain;
use crate::engine::scene::water::Water;
use crate::engine::scene::world::system::current_system::CurrentSystem;
use crate::engine::scene::world::system::update_focal_radius_system::UpdateFocalRadiusSystem;
use crate::engine::scene::world::system::update_god_rays_system::UpdateGodRaysSystem;
use crate::engine::texture::resource_provider::ResourceProvider;
use crate::engine::transformable::Transformable;

use super::background::Background;
use super::world::component::camera_component::CameraComponent;
use super::world::component::collider_component::ColliderComponent;
use super::world::component::conditional_parent_component::{
    AnimationCondition, ConditionalParentComponent,
};
use super::world::component::emitter_component::EmitterComponent;
use super::world::component::model_component::ModelComponent;
use super::world::component::movement_component::MovementComponent;
use super::world::component::multi_conditional_parent_component::MultiConditionalParentComponent;
use super::world::component::parent_component::ParentComponent;
use super::world::component::player_part_component::PlayerPartComponent;
use super::world::component::player_state_component::PlayerStateComponent;
use super::world::component::texture_component::TextureComponent;
use super::world::component::transform_component::TransformComponent;
use super::world::entity::entity::Entity;
use super::world::entity::particle_entity::ParticleEntityType;
use super::world::system::animation_system::AnimationSystem;
use super::world::system::camera_system::CameraSystem;
use super::world::system::collider_system::ColliderSystem;
use super::world::system::emitter_system::EmitterSystem;
use super::world::system::input_system::InputSystem;
use super::world::system::movement_system::MovementSystem;
use super::world::system::particle_system::ParticleSystem;
use super::world::system::player_movement_system::PlayerMovementSystem;
use super::world::system::render_system::RenderSystem;
use super::world::system::system::System;
use super::world::system::terrain_system::TerrainSystem;
use super::world::world::World;

pub struct Scene {
    pub systems: Vec<Box<dyn System>>,
    world: World,
}

const WORLD_SEED: u32 = 696969;

impl Scene {
    pub fn new(resource_provider: &mut dyn ResourceProvider) -> Self {
        let mut world = World::load();
        let water = Water::create((WORLD_SEED ^ 0x5EAF00D).wrapping_mul(69696969));
        let terrain = Terrain::generate(&mut world, WORLD_SEED, resource_provider);
        world.insert_resource(Arc::new(Mutex::new(terrain)));
        world.insert_resource(water);
        world.insert_resource(Background::construct(resource_provider));
        let model_scale = 0.15;
        let initial_position = Vec3::new(0.0, 0.25, 0.0);

        let player = world.create_entity();
        let camera = world.create_entity();

        world.add_component(
            camera,
            CameraComponent {
                position: Vec3::new(0.0, 0.25, 0.0),
                move_speed: 0.69,
                zoom_speed: 0.1,
                near: 0.0,
                far: 10.0,
                focal_offset: Vec2::new(0.0, 0.0),
                max_distance_from_player: 0.25,
                target_entity: Some(player),
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
        let left_hand_texture = resource_provider
            .load_static_texture("player/left_hand.png")
            .unwrap();
        let legs_texture = resource_provider
            .load_static_texture("player/legs0.png")
            .unwrap();
        let torso_texture = resource_provider
            .load_static_texture("player/torso.png")
            .unwrap();
        let right_hand_texture = resource_provider
            .load_static_texture("player/right_hand.png")
            .unwrap();
        let tank_texture = resource_provider
            .load_static_texture("player/tank.png")
            .unwrap();
        let head_texture = resource_provider
            .load_animated_texture(head_textures.as_slice(), 6000)
            .unwrap();
        let moving_legs_texture = resource_provider
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

        let pattern_model = resource_provider.get_model("square");

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
            world.add_component::<ModelComponent>(
                *child,
                pattern_model.clone().get_mesh().clone().into(),
            );
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

        let bubble_model = resource_provider.get_model("bubble");
        let bubble_emitter = world.create_entity();
        world.add_component::<EmitterComponent>(bubble_emitter, ParticleEntityType::Bubble.into());
        world.add_component::<TransformComponent>(
            bubble_emitter,
            TransformComponent {
                position: (0.025, -0.025, 0.0001).into(),
                rotation: bubble_model.get_rotation(),
                scale: bubble_model.get_scale(),
                is_flipped: bubble_model.is_flipped(),
            },
        );
        world.add_component::<ModelComponent>(
            bubble_emitter,
            bubble_model.get_mesh().clone().into(),
        );
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
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(InputSystem),
            Box::new(PlayerMovementSystem),
            Box::new(MovementSystem),
            Box::new(TerrainSystem),
            Box::new(CameraSystem),
            Box::new(AnimationSystem),
            Box::new(ParticleSystem),
            Box::new(EmitterSystem),
            Box::new(ColliderSystem),
            Box::new(CurrentSystem),
            Box::new(UpdateFocalRadiusSystem),
            Box::new(UpdateGodRaysSystem),
            Box::new(RenderSystem),
        ];
        Scene { systems, world }
    }

    pub fn update(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.run(&mut self.world, delta_time);
        }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
