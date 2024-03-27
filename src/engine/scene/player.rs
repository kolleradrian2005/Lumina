use std::{
    cell::RefCell,
    f32::consts::PI,
    rc::Rc,
    time::{Duration, Instant},
    u128,
};

use glfw::Key;

use crate::engine::{
    collider::Collider,
    input_handler::InputHandler,
    math::{vec2::Vec2, vec3::Vec3},
    model::{model::Model, model_group::ModelGroup},
    texture::{resource_manager::ResourceManager, texture::Texture},
    transformable::Transformable,
};

use super::{
    particle::{particle::ParticleType, particle_system::ParticleSystem},
    world::World,
};

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Swimming,
    FastSwimming,
}

impl PlayerState {
    pub const fn zoom(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming => 0.02,
            PlayerState::FastSwimming => 0.05,
        }
    }
    pub const fn acceleration(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming => 1.0,
            PlayerState::FastSwimming => 1.25,
        }
    }
    pub const fn is_swimming(&self) -> bool {
        match self {
            PlayerState::Idle => false,
            PlayerState::Swimming => true,
            PlayerState::FastSwimming => true,
        }
    }
    pub const fn legs_animation_time(&self) -> u128 {
        match self {
            PlayerState::Idle => 0,
            PlayerState::Swimming => 350,
            PlayerState::FastSwimming => 300,
        }
    }
    pub const fn light_level(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.225,
            PlayerState::Swimming => 0.175,
            PlayerState::FastSwimming => 0.1,
        }
    }
}
pub struct Player {
    pub model_group: ModelGroup,
    initial_models: Vec<Model>, // Model palette
    dest_rotation: f32,
    acceleration: Vec3,
    velocity: Vec3,
    state: PlayerState,
    pub initial_positions: Vec<Vec3>,
    pub initial_scales: Vec<f32>,
    last_exhale_time: Instant,
    particle: Option<Rc<RefCell<ParticleSystem>>>,
}

/*
Model group indices:

    0 - left hand
    1 - legs
    2 - torso
    3 - right hand
    4 - tank
    5 - head

*/

impl Player {
    pub fn new(resource_manager: &mut ResourceManager) -> Self {
        let model_scale = 0.15;
        let initial_position = Vec3::new(0.0, 0.25, 0.0);

        let mut model_group =
            ModelGroup::new(Collider::rect(0.4, 1.4, (-0.05, -0.05).into()).into());
        model_group.set_scale(Vec2::new(model_scale, model_scale));
        model_group.set_position(initial_position);

        let mut pattern_model = resource_manager.get_model("square");
        pattern_model.set_scale(Vec2::new(2.0, 2.0));

        let mut head_model = pattern_model.clone();
        let mut torso_model = pattern_model.clone();
        let mut left_hand_model = pattern_model.clone();
        let mut right_hand_model = pattern_model.clone();
        let mut legs_model = pattern_model.clone();
        let mut tank_model = pattern_model.clone();
        let mut moving_legs_model = pattern_model.clone();

        let mut head_textures: Vec<&str> = vec!["./player/head2.png", "./player/head3.png"];

        let count = 22;
        for _ in 0..count {
            head_textures.push("./player/head0.png");
            head_textures.push("./player/head1.png");
        }

        let moving_legs_textures: &[&str] = &["./player/legs0.png", "./player/legs1.png"];

        let texture_handler = resource_manager.get_texture_handler_mut();

        let left_hand_texture = texture_handler.load_static_texture("./player/left_hand.png");
        let legs_texture = texture_handler.load_static_texture("./player/legs0.png");
        let torso_texture = texture_handler.load_static_texture("./player/torso.png");
        let right_hand_texture = texture_handler.load_static_texture("./player/right_hand.png");
        let tank_texture = texture_handler.load_static_texture("./player/tank.png");
        let head_texture = texture_handler.load_animated_texture(head_textures.as_slice(), 6000);
        let moving_legs_texture = texture_handler.load_animated_texture(
            moving_legs_textures,
            PlayerState::legs_animation_time(&PlayerState::Swimming),
        );

        left_hand_model.set_texture(left_hand_texture.unwrap());
        legs_model.set_texture(legs_texture.unwrap());
        torso_model.set_texture(torso_texture.unwrap());
        right_hand_model.set_texture(right_hand_texture.unwrap());
        tank_model.set_texture(tank_texture.unwrap());
        head_model.set_texture(head_texture.unwrap());
        moving_legs_model.set_texture(moving_legs_texture.unwrap());

        let initial_scales = vec![
            0.31640625, 0.4375, 0.23828125, 0.32421875, 0.25, 0.23828125, 0.4375,
        ];

        let initial_positions = vec![
            Vec3::from_vec2(Vec2::new(0.08984375, -0.03515625) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.03125, -0.3984375) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.05078125, 0.09765625) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.03515625, -0.05078125) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.2265625, 0.125) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.03515625, 0.54296875) * model_scale, 0.0),
            Vec3::from_vec2(Vec2::new(-0.03125, -0.3984375) * model_scale, 0.0),
        ];

        let mut initial_models: Vec<Model> = vec![
            left_hand_model,
            legs_model,
            torso_model,
            right_hand_model,
            tank_model,
            head_model,
            moving_legs_model,
        ];

        for (index, model) in initial_models.iter_mut().enumerate() {
            model.set_scale(model.get_scale() * initial_scales[index]);
            model.set_position(initial_positions[index]);
            // First five models are relevant
            if index <= 5 {
                model_group.add_model(model.clone());
            }
        }
        Player {
            model_group,
            initial_models,
            dest_rotation: 0.0,
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            state: PlayerState::Idle,
            initial_positions,
            initial_scales,
            last_exhale_time: Instant::now(),
            particle: None,
        }
    }

    pub fn get_state(&self) -> &PlayerState {
        &self.state
    }

    pub fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    fn calculate_particle_position(
        position: Vec3,
        rotation: f32,
        is_flipped: bool,
        head_position: Vec3,
        head_rotation: f32,
    ) -> Vec3 {
        let mut particle_position2d = Vec2::new(0.025, -0.025);
        if is_flipped {
            particle_position2d.x *= -1.0;
        }
        particle_position2d.rotate(head_rotation);
        particle_position2d += head_position.xy();
        particle_position2d.rotate(rotation);
        particle_position2d += position.xy();
        Vec3::from_vec2(particle_position2d, 0.0)
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        input_handler: &InputHandler,
        resource_manager: &mut ResourceManager,
        water_resistance: f32,
        world: &World,
        particles: &mut Vec<Rc<RefCell<ParticleSystem>>>,
    ) {
        // Change position based on input
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        // W pressed
        if input_handler.is_pressed(Key::W) {
            direction.y += 1.0;
        }
        // A pressed
        if input_handler.is_pressed(Key::A) {
            direction.x -= 1.0;
        }
        // S pressed
        if input_handler.is_pressed(Key::S) {
            direction.y -= 1.0;
        }
        // D pressed
        if input_handler.is_pressed(Key::D) {
            direction.x += 1.0;
        }
        let direction_normal = direction.normalized();
        let mut new_state = PlayerState::Idle;
        if 0.0 < direction.length() {
            if input_handler.is_pressed(Key::LeftShift) {
                new_state = PlayerState::FastSwimming;
            } else {
                new_state = PlayerState::Swimming;
            }
        }
        if self.state != new_state {
            let model_group = &mut self.model_group;
            let legs_index = 1;
            let moving_legs_index = 6;
            if !self.state.is_swimming() && new_state.is_swimming() {
                let mut new_model = self.initial_models[moving_legs_index].clone();
                if let Texture::AnimatedTexture(animated_texture) = new_model.get_texture_mut() {
                    animated_texture.animation_time = new_state.legs_animation_time();
                    model_group.replace_model(legs_index, new_model);
                }
            } else if new_state.is_swimming() {
                if let Some(model) = model_group.get_model_mut(legs_index) {
                    if let Texture::AnimatedTexture(animated_texture) = model.get_texture_mut() {
                        animated_texture.animation_time = new_state.legs_animation_time();
                    }
                }
            } else {
                model_group.replace_model(legs_index, self.initial_models[legs_index].clone());
            }
            self.state = new_state;
        }
        // Round velocity to zero when low and not moving
        let treshold = 0.002;
        if direction.x == 0.0 && self.velocity.x.abs() < treshold {
            self.velocity.x = 0.0;
        }
        if direction.y == 0.0 && self.velocity.y.abs() < treshold {
            self.velocity.y = 0.0;
        }
        // Calculate acceleration
        self.acceleration = direction_normal * self.state.acceleration();
        // Calculate velocity
        self.velocity += self.acceleration * delta_time;
        self.velocity *= 1.0 - water_resistance;
        //println!("{:?}", self.velocity);
        // Calculate offset
        let offset = self.velocity * delta_time;
        let new_position = self.model_group.get_position() + offset;
        self.model_group.set_position(new_position);
        // Rotation animation
        if direction_normal.length() == 0.0 || !self.state.is_swimming() {
            self.dest_rotation = 0.0;
        } else {
            self.dest_rotation = (-direction_normal.y).atan2(direction_normal.x) + PI / 2.0;
        }
        let mut rotation = self.model_group.get_rotation().clone();
        if self.dest_rotation < 0.0 {
            self.dest_rotation += 2.0 * PI;
        }
        self.dest_rotation %= 2.0 * PI;
        let mut difference = self.dest_rotation - rotation;
        if PI < difference {
            difference = difference - 2.0 * PI;
        } else if PI < -difference {
            difference = difference + 2.0 * PI;
        }
        let rot_speed = 5.0;
        rotation += rot_speed * difference * delta_time;
        rotation %= 2.0 * PI;
        if rotation < 0.0 {
            rotation += 2.0 * PI;
        }
        let head_index = 5;
        let is_flipped = !(0.0 <= rotation && rotation <= PI);
        let initial_head_position = self.initial_positions[head_index];
        let model_group = &mut self.model_group;
        model_group.set_flipped(is_flipped);
        model_group.set_rotation(rotation);
        // Move state
        let mul = ((is_flipped as i32) * 2 - 1) as f32;
        if let Some(head) = model_group.get_model_mut(head_index) {
            if self.state.is_swimming() {
                let mut head_position = head.get_position().clone();
                if !is_flipped {
                    head_position.x = initial_head_position.x - 0.01;
                } else {
                    head_position.x = initial_head_position.x + 0.02;
                }
                head_position.y = initial_head_position.y - 0.01;
                head.set_position(head_position);
                head.set_rotation(mul * PI / 2.0);
            } else {
                let mut new_position = initial_head_position.clone();
                if is_flipped {
                    new_position.x *= -1.0;
                }
                head.set_position(new_position);
                head.set_rotation(0.0);
            }
            let head_rotation = head.get_rotation();
            let head_position = head.get_position();
            let now = Instant::now();
            if let Some(particle_ptr) = &mut self.particle {
                if let Ok(mut particle) = RefCell::try_borrow_mut(&particle_ptr) {
                    let particle_position: Vec3 = Self::calculate_particle_position(
                        new_position,
                        rotation,
                        is_flipped,
                        head_position,
                        head_rotation,
                    );
                    particle.set_spawn_position(particle_position);
                }
            }
            if 6.0 < now.duration_since(self.last_exhale_time).as_secs_f32() {
                let particle_position: Vec3 = Self::calculate_particle_position(
                    new_position,
                    rotation,
                    is_flipped,
                    head_position,
                    head_rotation,
                );
                let mut bubble_particle = ParticleSystem::spawn(
                    ParticleType::Bubble,
                    particle_position,
                    resource_manager,
                );
                bubble_particle.set_lifespan(Some(Duration::from_secs_f32(3.0)));
                bubble_particle.set_particle_lifespan(Some(Duration::from_secs_f32(4.0)));
                let particle_ptr = Rc::new(RefCell::new(bubble_particle));
                self.particle = Some(particle_ptr.clone());
                particles.push(particle_ptr);
                self.last_exhale_time = now;
            }
        }

        if let Some(collider) = model_group.get_collider() {
            let mut y_offset = 0.0;
            for point in collider.transformed_points {
                let height = world.get_terrain().get_height(point.x);
                if point.y < height {
                    y_offset = f32::max(y_offset, height - point.y);
                }
            }
            let mut pos = model_group.get_position();
            pos.y += y_offset;
            model_group.set_position(pos);
        }
    }
}
