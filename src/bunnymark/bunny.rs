use amethyst::{
    core::cgmath::Vector3,
    ecs::{Component, DenseVecStorage},
};

pub struct Bunny {
    pub velocity: Vector3<f32>,
}

impl Component for Bunny {
    type Storage = DenseVecStorage<Self>;
}
