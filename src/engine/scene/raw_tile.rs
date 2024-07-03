pub struct RawTile {
    vertices: [f32],
}

impl RawTile {
    pub fn generate(
        size: f32,
        x: i32,
        noise: &Perlin,
        texture: &Texture,
        resource_manager: &mut ResourceManager,
    ) -> Tile {
        // Generate model

        let z_index = 0.0;
        let previous_y: f32 = Terrain::get_height_noise(x - 1, noise);
        let current_y: f32 = Terrain::get_height_noise(x, noise);
        let next_y: f32 = Terrain::get_height_noise(x + 1, noise);

        let left_y_offset = (previous_y - current_y) / 2.0;
        let right_y_offset = (current_y - next_y) / 2.0;
        let a = size / 2.0 + left_y_offset;
        let b = size / 2.0 - right_y_offset;
        let uphill = a < b;
        let top = f32::max(previous_y, next_y);
        let bot = f32::min(previous_y, next_y);

        let height = top - bot;

        let vertices: [f32] = [
            // Bottom left
            -size / 2.0,
            bot,
            sprite::Z_DEFAULT,
            // Bottom right
            size / 2.0,
            bot,
            sprite::Z_DEFAULT,
            // Top right
            size / 2.0,
            top,
            sprite::Z_DEFAULT,
            // Top left
            -size / 2.0,
            top,
            sprite::Z_DEFAULT,
        ];
        //let mut tile_model = Model::new(vertices, &sprite::INDICES, &sprite::UVS);
        tile_model.set_texture(texture.clone());
        let tile_position = Vec3::new(x as f32 * size, current_y, z_index);
        tile_model.set_position(tile_position);

        // Generate objects
        let mut objects = Vec::new();

        // Generate seaweed
        let seaweed_per_chunk = 20;
        for i in 0..seaweed_per_chunk {
            let seaweed_noise = Terrain::get_seaweed_noise(x * seaweed_per_chunk + i, noise);
            let seaweed_treshold = 1.25;
            if seaweed_treshold < seaweed_noise.abs() {
                let ratio = i as f32 / seaweed_per_chunk as f32;
                let x = tile_position.x - size / 2.0 + size * ratio;
                let mut position = Vec3::new(
                    x,
                    current_y
                        + bot
                        + height
                            * Terrain::interpolate(f32::from(!uphill), f32::from(uphill), ratio),
                    z_index,
                );
                position.z = z_index;
                let mut seaweed = resource_manager.get_model("seagrass");
                if let Texture::StaticTexture(texture) = seaweed.get_texture() {
                    position.y +=
                        seaweed.get_scale().y * texture.get_normalized_dimensions().1 / 2.0;
                }
                seaweed.set_position(position);
                objects.push(seaweed);
            }
        }
        Tile {
            model: tile_model,
            uphill,
            height: top - bot,
            objects,
        }
    }
}
