use thiserror::Error;


pub trait RenderObject: sealed::RenderObject {}
impl<T: sealed::RenderObject> RenderObject for T {}

mod sealed {
    pub trait RenderObject {}

    impl RenderObject for wgpu::BindGroupLayout {}
    impl RenderObject for wgpu::PipelineLayout {}
    impl RenderObject for wgpu::ShaderModule {}
    impl RenderObject for wgpu::RenderPipeline {}
    impl RenderObject for wgpu::ComputePipeline {}
}


pub trait RenderObjectDesc<'desc, C> {
    type Object: RenderObject;

    fn key(&self) -> &str;
    fn try_create(self, ctx: &C) -> RenderObjectResult<'desc, Self::Object>;
}


// TODO: Better names for result/error types
#[derive(Error, Debug)]
pub enum RenderObjectError<'key> {
    #[error("Missing dependency: {0}")]
    MissingDependency(&'key str)
}

pub type RenderObjectResult<'key, T> = Result<T, RenderObjectError<'key>>;
