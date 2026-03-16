use std::f32::consts::PI;

use lumina_engine::{
    logic::{
        ecs::{
            component::{material::Material, parent::Parent, transform::Transform},
            system::system::System,
        },
        scene::world::World,
    },
    render::resource::texture::texture::Texture,
};

use crate::components::{
    conditional_parent::{AnimationCondition, ConditionalParent},
    multi_conditional_parent::MultiConditionalParent,
    player_part::PlayerPart,
    player_state::PlayerState,
};

pub struct AnimationSystem;

impl System for AnimationSystem {
    fn run(&mut self, world: &mut World, delta_time: f32) {
        world
            .query_mut::<(&mut PlayerState, &mut Transform)>()
            .last()
            .map(|(_, (player_state, transform))| {
                Self::animate_player(world, player_state, transform, delta_time);
            });
    }
}

impl AnimationSystem {
    fn animate_player(
        world: &mut World,
        player_state: &mut PlayerState,
        transform: &mut Transform,
        delta_time: f32,
    ) {
        let direction_normal = player_state.direction().normalized();
        let mut dest_rotation =
            match direction_normal.length() == 0.0 || !player_state.is_swimming() {
                true => 0.0,
                false => (-direction_normal.y).atan2(direction_normal.x) + PI / 2.0,
            };
        if dest_rotation < 0.0 {
            dest_rotation += 2.0 * PI;
        }
        dest_rotation %= 2.0 * PI;
        let mut difference = dest_rotation - transform.rotation;
        if PI < difference {
            difference = difference - 2.0 * PI;
        } else if PI < -difference {
            difference = difference + 2.0 * PI;
        }
        let rot_speed = 5.0;
        transform.rotation += rot_speed * difference * delta_time;
        transform.rotation %= 2.0 * PI;
        if transform.rotation < 0.0 {
            transform.rotation += 2.0 * PI;
        }
        transform.is_flipped = !(0.0 <= transform.rotation && transform.rotation <= PI);

        for (_, (parent, multi_conditional_parent)) in
            world.query_mut::<(&mut Parent, &mut MultiConditionalParent)>()
        {
            for conditional_parent in multi_conditional_parent.components.iter_mut() {
                if Self::bind_parent(parent, conditional_parent, &player_state) {
                    break;
                }
            }
        }

        for (_, (player_part, material_component)) in
            world.query_mut::<(&mut PlayerPart, &mut Material)>()
        {
            if let Texture::AnimatedTexture(texture) = &mut material_component.texture {
                if let PlayerPart::Legs = player_part {
                    texture.animation_time = player_state.legs_animation_time();
                }
            }
        }
    }
    fn bind_parent(
        parent: &mut Parent,
        conditional_parent: &mut ConditionalParent,
        player_state: &PlayerState,
    ) -> bool {
        if let AnimationCondition::True = conditional_parent.condition {
            parent.parent = conditional_parent.parent;
            return true;
        }
        return match conditional_parent.condition {
            AnimationCondition::PlayerIdle => match player_state {
                PlayerState::Idle => {
                    parent.parent = conditional_parent.parent;
                    true
                }
                _ => false,
            },
            AnimationCondition::PlayerSwimming => match player_state {
                PlayerState::Idle => return false,
                _ => {
                    parent.parent = conditional_parent.parent;
                    true
                }
            },
            _ => false,
        };
    }
}
