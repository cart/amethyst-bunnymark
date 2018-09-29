use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
};
use bunnymark::{MoveBunniesSystem, SpawnBunniesSystem};

pub struct BunnyMarkBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BunnyMarkBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add_thread_local(MoveBunniesSystem::new());
        builder.add_thread_local(SpawnBunniesSystem::new());
        Ok(())
    }
}
