use bunnymark::bunny_resource::BunnyResource;
use amethyst::{
    core::{
        cgmath::{Matrix4, Vector3},
        GlobalTransform,
    },
    ecs::Entity,
    prelude::*,
    renderer::{Camera, Event, Projection, WindowEvent},
    winit::dpi::LogicalSize,
};

pub struct BunnyMark {
    pub camera: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, ()> for BunnyMark {
    fn on_start(&mut self, data: StateData<GameData>) {
        BunnyResource::initialize(data.world);

        let bounds = data.world.read_resource::<BunnyResource>().bounds;

        self.camera = Some(
            data.world
                .create_entity()
                .with(Camera::from(Projection::orthographic(
                    0.0, bounds.x, bounds.y, 0.0,
                )))
                .with(GlobalTransform(
                    Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.1)).into(),
                ))
                .build(),
        );
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, ()> {
        data.data.update(&data.world);
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: StateEvent<()>,
    ) -> Trans<GameData<'a, 'b>, ()> {
        match event {
            StateEvent::Window(event) => match event {
                Event::WindowEvent {
                    window_id: _,
                    event: window_event,
                } => match window_event {
                    WindowEvent::Resized(logical_size) => {
                        let mut bunny_resources = data.world.write_resource::<BunnyResource>();

                        let LogicalSize { width, height } = logical_size;
                        let mut cameras = data.world.write_storage::<Camera>();
                        let mut camera = cameras.get_mut(self.camera.unwrap()).unwrap();
                        camera.proj =
                            Projection::orthographic(0.0, width as f32, height as f32, 0.0).into();
                        bunny_resources.bounds.x = width as f32;
                        bunny_resources.bounds.y = height as f32;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        Trans::None
    }
}
