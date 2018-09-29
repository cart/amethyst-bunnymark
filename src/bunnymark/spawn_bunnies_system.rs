use amethyst::{
    core::{cgmath::Vector3, GlobalTransform, Time, Transform},
    ecs::{
        prelude::{Read, System},
        Entities, ReadExpect, WriteStorage,
    },
    renderer::{Sprite, SpriteRender, TextureCoordinates},
};
use bunnymark::{BunnyResource, Bunny};
use rand::random;

const SPAWN_WAIT_TIME_IN_SECONDS: f32 = 0.5;
const BUNNIES_TO_SPAWN: i32 = 100;

pub struct SpawnBunniesSystem {
    pub elapsed: f32,
    pub print_elapsed: f32,
    pub count: usize,
    pub sprite: Sprite,
}

impl SpawnBunniesSystem {
    pub fn new() -> SpawnBunniesSystem {
        SpawnBunniesSystem {
            elapsed: 0.0,
            print_elapsed: 0.0,
            count: 0,
            sprite: Sprite {
                height: 604.0,
                width: 604.0,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 0.0,
                },
            },
        }
    }

    fn spawn_bunny<'s>(
        &mut self,
        entities: &Entities<'s>,
        bunnies: &mut WriteStorage<'s, Bunny>,
        transforms: &mut WriteStorage<'s, Transform>,
        global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        sprite_renders: &mut WriteStorage<'s, SpriteRender>,
        bunny_resource: &ReadExpect<'s, BunnyResource>,
    ) {
        let mut transform = Transform::default();
        transform.translation = Vector3::new(
            bunny_resource.bounds.x / 2.0 - bunny_resource.sprite_size.x / 2.0,
            bunny_resource.bounds.y / 2.0 - bunny_resource.sprite_size.y / 2.0,
            0.0,
        );

        let sprite_render = SpriteRender {
            sprite_sheet: bunny_resource.sprite_sheet.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        entities
            .build_entity()
            .with(sprite_render, sprite_renders)
            .with(
                Bunny {
                    velocity: Vector3::new(
                        random::<f32>() * 200.0 + 50.0,
                        random::<f32>() * 200.0 + 50.0,
                        0.0,
                    ),
                },
                bunnies,
            )
            .with(transform, transforms)
            .with(GlobalTransform::default(), global_transforms)
            .build();

        self.count += 1;
    }
}

impl<'s> System<'s> for SpawnBunniesSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bunny>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, GlobalTransform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, BunnyResource>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut bunnies,
            mut transforms,
            mut global_transforms,
            mut sprite_renders,
            bunny_resources,
            time,
        ) = data;

        self.elapsed += time.delta_seconds();
        self.print_elapsed += time.delta_seconds();
        
        if self.elapsed > SPAWN_WAIT_TIME_IN_SECONDS {
            for _ in 0..BUNNIES_TO_SPAWN {
                self.spawn_bunny(
                    &entities,
                    &mut bunnies,
                    &mut transforms,
                    &mut global_transforms,
                    &mut sprite_renders,
                    &bunny_resources,
                );
            }
            self.elapsed = 0.0;
        }

        if self.print_elapsed > 2.0 {
            println!("{}", self.count);
            self.print_elapsed = 0.0;
        }
    }
}
