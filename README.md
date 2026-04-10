# Lumina

A thesis-focused 2D game engine and aquatic exploration game built in Rust. The project demonstrates a clean separation between a reusable rendering/ECS engine (`lumina_engine`) and game-specific logic (`lumina_game`).

## Project Structure

```
Lumina/
├── engine/       # lumina_engine — game-agnostic core
├── game/         # lumina_game — the aquatic exploration game
└── macro_lib/    # lumina_macro — proc-macro crate (#[derive(Component)])
```

## Engine (`lumina_engine`)

A minimal 2D engine built on top of OpenGL (GLSL 450 core), `glutin`/`winit` for windowing, and `flume` for cross-thread communication.

### Core Modules

- **ECS** — `World` stores entities, typed component storages (`HashMap<TypeId, …>`), and arbitrary resources (`HashMap<TypeId, Box<dyn Any>>`). Queries are typed and support both shared and mutable access.
- **Rendering** — `GenericRenderer` issues OpenGL draw calls per `RenderEntity`. Supports triangles, lines, and tessellation patches. A `Framebuffer` with optional MSAA (16×) enables a full-screen post-processing pass.
- **Scene** — `Scene` owns the `World`, a list of `Box<dyn System>`, and a list of `Box<dyn Extractor>`. Each frame: systems update the world, extractors collect a `ExtractedFrame`, which is sent to the render thread via a bounded channel.
- **Math** — `Vec2`, `Vec3`, affine `Transform`, rect/capsule collision geometry.
- **Input** — `InputState` resource updated from `winit` events forwarded through a channel from the main thread.
- **Resource Management** — `ResourceManager` loads textures, meshes, and GLSL shaders from compile-time embedded asset archives (`include_assets`). Communicates with the render thread for GPU-side uploads.

### Threading Model

```
Main thread (winit event loop)
  │  input events ──► input_tx
  │  resource commands ──► resource_tx
  │  ExtractedFrame ◄── render_rx
  └► OpenGL draw calls

Logic thread (spawned once)
  ├─ receives input events from input_rx
  ├─ runs Scene::update (all Systems, 60 Hz target)
  ├─ runs Scene::extract (all Extractors)
  └─ sends ExtractedFrame via render_tx
```

### Built-in Components

| Component   | Purpose                                              |
| ----------- | ---------------------------------------------------- |
| `Transform` | Position (Vec3), scale (Vec2), rotation (f32)        |
| `Movement`  | Velocity, acceleration                               |
| `Force`     | Mass + applied linear/drag forces (Newton 2nd law)   |
| `Material`  | Shader handle, texture, typed uniform parameters     |
| `Model`     | Mesh reference + object-type metadata                |
| `Collider`  | Rect or Capsule2D shape with SAT intersection        |
| `Camera`    | Orthographic projection, position, near/far          |
| `Emitter`   | Particle emitter with configurable interval/lifespan |
| `Parent`    | Entity hierarchy link                                |

### Built-in Systems & Extractors

**Systems** (run each frame in order):
`MovementSystem` → `ParticleSystem` → `EmitterSystem` → `CollisionSystem` → `DebugSystem`

**Extractors** (collect render data each frame):
`ModelExtractor`, `ParticleExtractor`, `DebugExtractor`, `PostprocessExtractor`

### Post-Processing

The engine supports one full-screen post-process pass per frame. A `PostprocessConfig` resource carries a `Material` whose shader receives:

- A `MatrixUniformBuffer` (projection + view matrices, std140)
- A `PostProcessUniformBuffer` (saturation, tint, vignette, focal radius, smooth factor)
- Per-frame dynamic uniforms (focal offset, aspect ratio, light positions for god rays)

### Shaders

All GLSL shaders are embedded at compile time. The engine ships:

- `model.vert/frag` — standard sprite/model rendering
- `model.tesc/tese` — tessellation shaders (e.g. seagrass wave deformation driven by a `uCurrent` uniform)
- `postprocess.vert/frag` — full-screen post-processing (focal blur, vignette, tint, god rays)
- `debug.vert/frag` — debug geometry overlay

### Exported API Surface

```rust
// Start the engine (called from game crate)
lumina_engine::app::start(event_loop, |scene, resource_manager| {
    // load resources, spawn entities, register systems
});

// World — entity/component/resource management
let entity = world.create_entity();
world.add_component(entity, Transform { … });
world.insert_resource(MyResource { … });
for (_, (transform, movement)) in world.query_mut::<(&mut Transform, &mut Movement)>() { … }

// Scene — system/extractor registration
scene.register_system(Box::new(MySystem));
scene.register_extractor(Box::new(MyExtractor));

// Custom system
impl System for MySystem {
    fn run(&mut self, world: &mut World, delta_time: f32) { … }
}

// Custom component (proc-macro)
#[derive(Component)]
pub struct MyComponent { … }
```

## Game (`lumina_game`)

An aquatic exploration game that demonstrates the engine's extensibility.

### Player

The player is a **multi-part entity hierarchy** (body parts linked via `Parent` components). Movement is driven by a `PlayerState` enum:

| State          | Acceleration | Drag (via Water)      |
| -------------- | ------------ | --------------------- |
| `Idle`         | 0.0          | full water resistance |
| `Swimming`     | 1.0 × mass   | 0.9 drag factor       |
| `FastSwimming` | 1.25 × mass  | 0.9 drag factor       |

`InputSystem` → `PlayerMovementSystem` → `Force` component → engine `MovementSystem` applies Newton's 2nd law every frame.

`AnimationSystem` smoothly rotates the player body toward the movement direction and swaps sprite textures based on state and elapsed time.

### Water Physics

The `Water` resource holds:

- A **drag coefficient** (0.9) applied as a continuous `ForceEffect::Drag` on the player's `Force` component.
- A **Perlin-noise water current** sampled at the player's world position + time, yielding a horizontal force applied via `CurrentSystem`.

### Terrain

`Terrain` generates an infinite horizontal seagrass floor using **Perlin noise**. A `VecDeque<Tile>` acts as a sliding window: as the camera moves, tiles at one end are despawned and new tiles are generated at the other. Tile collision is handled by `TerrainCollisionSystem` using the engine's `Collider` + `CollisionSystem`.

### Camera & Post-Processing

`FollowSystem` moves the `Camera` entity toward the player. `CameraSystem` computes the orthographic projection + view matrices and uploads them to the `MatrixUniformBuffer`.

Post-processing uniforms are updated each frame:

- `UpdateFocalRadiusSystem` — adjusts the focal blur radius based on player state.
- `UpdateGodRaysSystem` — maintains a sliding window of Perlin-noise god-ray light positions relative to the camera, passed as `uLightPositions[MAX_LIGHTS]` to the post-process shader.

### Game-Specific Systems (registration order)

```
InputSystem → PlayerMovementSystem → CurrentSystem → TerrainSystem
→ FollowSystem → CameraSystem → AnimationSystem → TerrainCollisionSystem
→ UpdateFocalRadiusSystem → UpdateGodRaysSystem
```

### Assets

Assets are embedded at compile time (`include_dir!`) into each crate's binary. The game attaches its own `NamedArchive` on top of the engine's default archive, so both sets of textures and shaders are available through `ResourceManager`.

## Building & Running

### Prerequisites

- Rust 1.75+
- GPU with OpenGL 4.5 support

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Run Tests

```bash
cargo test --package lumina_engine
```

## Architecture Summary

```
Engine (game-agnostic)           Game (domain-specific)
─────────────────────────────    ──────────────────────────────
World / ECS core                 PlayerState, PlayerPart
Force / MovementSystem           PlayerMovementSystem, InputSystem
Collider / CollisionSystem       Terrain, TerrainCollisionSystem
Emitter / ParticleSystem         Particle (bubbles)
Camera / MatrixUniformBuffer     CameraSystem, FollowSystem
PostprocessConfig                Water, AnimationSystem
GenericRenderer / Framebuffer    UpdateGodRaysSystem
ResourceManager                  UpdateFocalRadiusSystem
```

**Key principle:** The engine exposes traits (`System`, `Extractor`, `ResourceProvider`) and data types; the game provides all concrete implementations. The engine has zero knowledge of water, terrain, or player concepts.

// In systems
let resource = world.get_resource::<CustomResource>();

````

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
````

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
