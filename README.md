# Lumina

A modular, thesis-focused game engine and aquatic exploration game built in Rust. The project demonstrates a separation between a reusable rendering/ECS engine and game-specific logic.

## Project Structure

```
Lumina/
├── engine/          # Game engine (core, game-agnostic)
└── game/            # Game implementation (Lumina game)
```

## Engine

A minimal 2D rendering engine with:

### Core Systems

- **ECS (Entity Component System)** — Flexible entity/component/system architecture with priority-based execution
- **Rendering** — OpenGL-based renderer with post-processing, GUI, and scene management
- **Math** — Vector types (Vec2, Vec3), transformations, collision geometry
- **Input Handling** — Keyboard, mouse, and touch input abstraction
- **Resource Management** — Texture, model, font, and shader asset loading

### Architecture Highlights

- **System Registry** — Extensible system ordering with priorities (INPUT=0, RENDER=999)
- **Physics Environment Trait** — Game-agnostic physics abstraction (engine doesn't depend on game concepts)
- **World Layers** — Clean separation for static/dynamic world components (background, terrain, water, foreground)
- **Component-Based Design** — Minimal coupling, extensible behavior

### Exported API

- `Scene` — Main world container (entities, systems, layers, resources)
- `World` — ECS core (entity creation, component management, queries)
- `System` trait — For custom behavior
- `ResourceProvider` trait — For custom asset loading
- `PhysicsEnvironment` trait — For custom physics (resistance, currents, etc.)

## Game

An aquatic exploration game demonstrating engine usage:

### Features

- **Player Character** — Animated multi-part character (head, torso, arms, tank, legs)
- **Procedural Terrain** — Infinite tile-based seagrass terrain generation
- **Water Physics** — Resistance and water currents affecting movement
- **Particle System** — Bubble particles emitted from player
- **Camera System** — Follow-cam with focal-area post-processing
- **God Rays** — Dynamic light rays based on water surface

### Game-Specific Systems

- `PlayerMovementSystem` — Player input and movement
- `AnimationSystem` — Texture-based animation states
- `TerrainSystem` — Dynamic terrain tile loading/unloading
- `CurrentSystem` — Water current visual effects
- `UpdateFocalRadiusSystem` — Post-processing focal area
- `UpdateGodRaysSystem` — God ray computation

### Assets

- Embedded asset archives (textures, shaders, fonts) per crate
- Modular resource loading (engine defaults + game-specific assets)

## Architecture

### Engine-Game Separation

```
Engine (Agnostic)              Game (Domain-Specific)
├─ MovementSystem              ├─ Water resource
├─ PhysicsEnvironment trait    ├─ GamePhysicsEnvironment impl
├─ Rendering pipeline          ├─ Player character
├─ ECS core                    ├─ Terrain generation
└─ UI framework                └─ Game systems
```

**Key Principle:** Engine provides abstractions (traits); game provides implementations (concrete types).

- Engine systems don't know about Water, Terrain, or Player
- Game implements `PhysicsEnvironment` trait to integrate with engine physics
- Game registers custom systems and entities via engine Scene API

## Building & Running

### Prerequisites

- Rust 1.75+
- OpenGL 4.5+ support (or OpenGL ES 3.0+ on mobile)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --bin lumina_game --release
```

### Run Tests

```bash
cargo test --package lumina_engine
```

## Technical Details

### Threading Model

- **Main thread** — Window events, rendering, GL commands
- **Worker thread(s)** — ECS system execution, game logic
- **Render Queue** — Thread-safe data preparation for rendering

### Rendering Pipeline

1. **Update Phase** — All systems run (prepare render data)
2. **Drain Phase** — Collect render entities from layers and entities
3. **Render Phase** — Execute GPU commands (main thread only)
4. **Post-Processing** — Apply effects (focal blur, god rays, vignette)

### ECS Queries

```rust
// Immutable query
for (entity, (transform, model)) in world.query::<(&TransformComponent, &ModelComponent)>() {
    // ...
}

// Mutable query
for (entity, (mut movement, mut transform)) in world.query_mut::<(&mut MovementComponent, &mut TransformComponent)>() {
    // ...
}
```

### System Priority Order

```
INPUT (0)
  ↓
PLAYER_LOGIC (10)
  ↓
MOVEMENT (20)
  ↓
TERRAIN (30)
  ↓
CAMERA (40)
  ↓
ANIMATION (50)
  ↓
PARTICLE (60)
  ↓
COLLISION (70)
  ↓
RENDER_PREP (90)
  ↓
RENDER (999)
```

## Extending the Engine

### Adding a Custom System

```rust
use lumina_engine::scene::world::system::System;

pub struct CustomSystem;

impl System for CustomSystem {
    fn run(&mut self, world: &mut World, delta_time: f32) {
        // Your game logic here
    }

    fn priority(&self) -> i32 {
        25 // Between MOVEMENT and TERRAIN
    }

    fn name(&self) -> &str { "CustomSystem" }

    fn as_any(&self) -> &dyn Any { self }
}

// Register in game initialization
scene.add_system(Box::new(CustomSystem));
```

### Adding a Custom Resource

```rust
#[derive(Clone)]
pub struct CustomResource {
    data: Vec<f32>,
}

// In game initialization
world.insert_resource(CustomResource { data: vec![1.0, 2.0, 3.0] });

// In systems
let resource = world.get_resource::<CustomResource>();
```

### Implementing Custom Physics

```rust
use lumina_engine::scene::physics_environment::PhysicsEnvironment;

pub struct CustomPhysicsEnv;

impl PhysicsEnvironment for CustomPhysicsEnv {
    fn get_resistance(&self, pos: Vec3) -> f32 {
        // Your physics logic
        0.5
    }

    fn as_any(&self) -> &dyn Any { self }
}

// Register in game initialization
world.insert_resource(Arc::new(custom_env) as Arc<dyn PhysicsEnvironment>);
```

## Performance Considerations

- **Component Queries** — Use mutable queries sparingly; prefer read-only when possible
- **Texture Streaming** — Terrain tiles load/unload based on player distance
- **Render Queue** — Data preparation on worker threads; GPU commands on main thread
- **Post-Processing** — Effects are composited efficiently via framebuffer objects

## Known Limitations

- Single player only
- 2D game world (3D rendering for visual depth)
- No networking
- No save/load system

## Future Work

- **Levels & Checkpoints** — Save player progress
- **Enemy AI** — NPC characters with behavior trees
- **Inventory System** — Item collection and management
- **Sound System** — Audio engine integration
- **Mobile Support** — Full Android/iOS support (currently PC/Linux/Mac only)

## License

This is an academic thesis project. See individual crate `Cargo.toml` files for dependencies and their licenses.

## Authors

Adrian Koller

---

For questions or issues, refer to the thesis documentation or examine the code comments throughout the engine and game modules.
