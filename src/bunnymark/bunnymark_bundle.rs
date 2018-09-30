use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
};
use bunnymark::{MoveBunniesSystem, SpawnBunniesSystem};

pub struct BunnyMarkBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BunnyMarkBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(MoveBunniesSystem::new(), "move_bunnies", &[]);
        builder.add(SpawnBunniesSystem::new(), "spawn_bunnies", &[]);
        Ok(())
    }
}
