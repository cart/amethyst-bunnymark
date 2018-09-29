use amethyst::ecs::ReadExpect;
use bunnymark::BunnyResource;
use amethyst::{
	core::{Time, Transform},
	ecs::prelude::{Join, Read, System, WriteStorage},
	utils::fps_counter::FPSCounter,
};
use bunnymark::Bunny;
use rand::random;

const GRAVITY: f32 = 500.0;

pub struct MoveBunniesSystem {
	elapsed: f32,
}

impl MoveBunniesSystem {
	pub fn new() -> MoveBunniesSystem {
		MoveBunniesSystem { elapsed: 0.0 }
	}
}

impl<'s> System<'s> for MoveBunniesSystem {
	type SystemData = (
		WriteStorage<'s, Bunny>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
		Read<'s, FPSCounter>,
		ReadExpect<'s, BunnyResource>,
	);

	fn run(&mut self, (mut bunnies, mut transforms, time, fps_counter, bunny_resource): Self::SystemData) {
		self.elapsed += time.delta_seconds();
		if self.elapsed > 2.0 {
			println!("{}", fps_counter.sampled_fps());
			self.elapsed = 0.0;
		}

		for (bunny, transform) in (&mut bunnies, &mut transforms).join() {
			let translation = transform.translation;
			let mut new_translation = transform.translation;
			bunny.velocity.y += GRAVITY * time.delta_seconds();

			if translation.x > bunny_resource.bounds.x {
				bunny.velocity.x *= -1.0;
			}

			if translation.x < 0.0 {
				bunny.velocity.x *= -1.0;
			}

			if translation.y > bunny_resource.bounds.y {
				new_translation.y = bunny_resource.bounds.y;
				if random::<f32>() > 0.5 {
					bunny.velocity.y = (random::<u32>() % 1100 + 50) as f32;
				} else {
					bunny.velocity.y *= -0.85;
				}
			}

			if translation.y < 0.0 {
				new_translation.y = 0.0;
				bunny.velocity[1] = 0.0;
			}

			transform.translation = new_translation + bunny.velocity * time.delta_seconds();
		}
	}
}
