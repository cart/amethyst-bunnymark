use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::cgmath::Vector2,
    ecs::World,
    renderer::{MaterialTextureSet, PngFormat, Sprite, SpriteSheet, Texture, TextureCoordinates},
    renderer::TextureMetadata
};

pub struct BunnyResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_size: Vector2<f32>,
    pub bounds: Vector2<f32>,
}

impl BunnyResource {
    pub fn initialize(world: &mut World) {
        let bunny_sprite = Sprite {
            height: 64.0,
            width: 64.0,
            offsets: [0.0, 0.0],
            tex_coords: TextureCoordinates {
                left: 0.0,
                right: 1.0,
                top: 1.0,
                bottom: 0.0,
            },
        };

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "icon.png",
                PngFormat,
                TextureMetadata::srgb(),
                (),
                &texture_storage,
            )
        };

        let texture_id = 0;

        {
            let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
            material_texture_set.insert(texture_id, texture_handle);
        }

        let sprite_sheet = SpriteSheet {
            texture_id: texture_id,
            sprites: vec![bunny_sprite],
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
        };

        let bunny_resources = BunnyResource {
            sprite_sheet: sprite_sheet_handle,
            bounds: Vector2 { x: 500.0, y: 500.0 },
            sprite_size: Vector2 { x: 64.0, y: 64.0 },
        };

        world.add_resource(bunny_resources);
    }
}