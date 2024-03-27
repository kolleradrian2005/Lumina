use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use rand::rngs::ThreadRng;
use rand::Rng;

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
use super::world::World;

pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
    pub player: Player,
    pub background: Background,
    pub foreground: Foreground,
    world: World,
    particles: Vec<Rc<RefCell<ParticleSystem>>>,
    last_fish_spawn: Instant,
}

const WORLD_SEED: u32 = 696969;

impl Scene {
    pub fn new(resource_manager: &mut ResourceManager) -> Self {
        Scene {
            models: Vec::new(),
            camera: Camera::new(),
            player: Player::new(resource_manager),
            background: Background::construct(resource_manager),
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
        input_handler: &InputHandler,
        resource_manager: &mut ResourceManager,
        rng: &mut ThreadRng,
        updatables: &mut Vec<Updatable>,
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
            let mut particle_system = RefCell::borrow_mut(particle_ptr);
            particle_system.update(delta_time, rng, self.world.get_terrain());
            particle_system.is_alive()
        });

        // Spawn fish
        const FISH_Z: f32 = -0.5;
        let now = Instant::now(); // TODO: dont use now and not: TODO: pass now down the stack anywhere needed

        // TODO: check if fish already spawned
        if 2.0 < now.duration_since(self.last_fish_spawn).as_secs_f32() {
            const POPULATION_TIME: f32 = 6.0;
            let sig = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
            let y_offset = rng.gen_range(0.0..=1.0) as f32 * 2.0 - 1.0;
            let player_position = self.player.model_group.get_position();
            let x = player_position.x + sig; // TODO: player.getPosition()
            let y = player_position.y + y_offset;
            let particle_position = Vec3::new(x, y, FISH_Z);
            let mut fish_particle =
                ParticleSystem::spawn(ParticleType::Fish, particle_position, resource_manager);
            fish_particle.set_lifespan(Duration::from_secs_f32(POPULATION_TIME).into());
            //fish_particle.set_particle_lifespan(Duration::from_secs_f32(14.0).into());
            fish_particle.set_particle_lifespan(None);
            fish_particle.set_particle_velocity(1.5);
            fish_particle.update(POPULATION_TIME, rng, self.world.get_terrain());
            let particle_ptr = Rc::new(RefCell::new(fish_particle));
            self.particles.push(particle_ptr);
            self.last_fish_spawn = now;
        }
    }

    pub fn get_particles(&self) -> &Vec<Rc<RefCell<ParticleSystem>>> {
        &self.particles
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }
}
