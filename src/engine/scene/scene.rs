use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use rand::rngs::StdRng;
use rand::Rng;

use crate::engine::command_queue::CommandQueue;
use crate::engine::input_handler::InputHandler;

use crate::engine::math::vec3::Vec3;
use crate::engine::model::model::Model;
use crate::engine::render::updatable::Updatable;
use crate::engine::scene::particle::particle::ParticleType;
use crate::engine::texture::resource_manager::ResourceManager;
use crate::engine::transformable::Transformable;

use super::background::Background;
use super::camera::Camera;
use super::foreground::Foreground;
use super::particle::particle_system::ParticleSystem;
use super::player::Player;
use super::world::world::World;

pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
    pub player: Player,
    pub background: Background,
    pub foreground: Foreground,
    world: World,
    particles: Vec<Arc<RwLock<ParticleSystem>>>,
    last_fish_spawn: Instant,
}

const WORLD_SEED: u32 = 696969;

impl Scene {
    pub fn new(command_queue: Arc<CommandQueue>, resource_manager: &mut ResourceManager) -> Self {
        Scene {
            models: Vec::new(),
            camera: Camera::new(),
            player: Player::new(resource_manager),
            background: Background::construct(command_queue, resource_manager),
            foreground: Foreground::construct(),
            world: World::load(WORLD_SEED, resource_manager),
            particles: Vec::new(),
            last_fish_spawn: Instant::now(),
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        now: Instant,
        input_handler: &InputHandler,
        resource_manager: &mut ResourceManager, // TODO: make optional (so that only physics get updated)
        rng: &mut StdRng,
        updatables: &mut VecDeque<Updatable>,
    ) {
        // Update player
        self.player.update(
            delta_time,
            &input_handler,
            resource_manager,
            self.world.get_water().get_resistance(),
            &self.world,
            &mut self.particles,
        );

        // Load terrain correctly
        let tile_index = (self.player.model_group.get_position().x
            / self.world.get_terrain().tile_size)
            .round() as i32;

        self.world
            .get_terrain_mut()
            .update_tile_index(tile_index, resource_manager);

        // Load godrays correctly
        self.foreground
            .update_god_rays(self.player.model_group.get_position().xy());

        // Update camera
        self.camera.update(delta_time, &mut self.player, updatables);

        // Update foreground
        self.foreground.update(delta_time, &self.player, updatables);

        // Update particles
        self.particles.retain_mut(|particle_ptr| {
            let mut particle_system = particle_ptr.write().unwrap();
            particle_system.update(delta_time, rng, self.world.get_terrain());
            particle_system.is_alive()
        });

        // Spawn fish
        const FISH_Z: f32 = -0.5;
        const POPULATION_TIME: f32 = 6.0;
        const X_DIST: f32 = 1.5;
        const X_SPEED: f32 = 2.5;
        const FISH_SPAWN_INTERVAL: f32 = 30.0;

        // TODO: check if fish already spawned
        if FISH_SPAWN_INTERVAL < now.duration_since(self.last_fish_spawn).as_secs_f32() {
            let sig = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
            let y_offset = rng.gen_range(0.0..=1.0) as f32 * 2.0 - 1.0;
            let player_position = self.player.model_group.get_position();
            let x = player_position.x + sig * X_DIST;
            let y = player_position.y + y_offset;
            let particle_position = Vec3::new(x, y, FISH_Z);
            let mut fish_particle =
                ParticleSystem::spawn(ParticleType::Fish, particle_position, resource_manager);
            fish_particle.set_model_flipped(sig == -1.0);
            fish_particle.set_lifespan(Duration::from_secs_f32(POPULATION_TIME).into());
            fish_particle.set_particle_lifespan(None);
            fish_particle.set_particle_velocity(sig * X_SPEED);
            fish_particle.update(POPULATION_TIME, rng, self.world.get_terrain());
            fish_particle.set_timeout(Duration::from_secs_f32(60.0).into());
            let particle_ptr = Arc::new(RwLock::new(fish_particle));
            self.particles.push(particle_ptr);
            self.last_fish_spawn = now;
        }
    }

    pub fn get_particles(&self) -> &Vec<Arc<RwLock<ParticleSystem>>> {
        &self.particles
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
