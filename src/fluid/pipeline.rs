use bevy::material::descriptor::BindGroupLayoutDescriptor;

pub trait FluidPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor;
}
