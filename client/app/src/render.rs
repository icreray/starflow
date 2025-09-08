use default::default;
use futures_lite::future;

use starflow_render::{
	assets::{desc::*, util::*, RenderAssetsCreation}, 
	Features, GpuContextConfig, Renderer
};
use starflow_window::WindowModule;


pub fn create_renderer<'w>(window: &WindowModule) -> Renderer<'w> {
	let context_config = GpuContextConfig::default()
		.add_features(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
	let mut renderer = future::block_on(
		Renderer::new(context_config, window.clone_handle())
	);
	renderer.create_assets(create_assets);
	renderer
}


fn create_assets(ctx: &mut RenderAssetsCreation) {
	ctx.create(BindGroupLayout::new("output_texture", &[
		binding(0)
			.visibility(ShaderStages::COMPUTE)
			.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly)
	])).unwrap();

	ctx.create(BindGroupLayout::new("input_texture", &[
		binding(0)
			.visibility(ShaderStages::FRAGMENT)
			.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly)
	])).unwrap();

	// main pass
	ctx.create(PipelineLayout {
		key: "main_pass",
		bind_group_layouts: &["output_texture"],
		push_constant_ranges: &[]
	}).unwrap();
	ctx.create(ShaderModule::new("main_pass",
		ShaderSource::Wgsl(include_str!("../../../assets/shaders/main_pass.wgsl").into())
	)).unwrap();

	ctx.create(ComputePipeline {
		key: "main_pass",
		layout: Some("main_pass"),
		module: "main_pass",
	}).unwrap();

	// blit
	ctx.create(ShaderModule::new("fullscreen",
		ShaderSource::Wgsl(include_str!("../../../assets/shaders/fullscreen.wgsl").into())
	)).unwrap();

	ctx.create(ShaderModule::new("blit",
		ShaderSource::Wgsl(include_str!("../../../assets/shaders/blit.wgsl").into())
	)).unwrap();

	ctx.create(PipelineLayout {
		key: "blit",
		bind_group_layouts: &["input_texture"],
		push_constant_ranges: &[]
	}).unwrap();

	ctx.create(RenderPipeline {
		key: "blit",
		layout: Some("blit"),
		vertex: "fullscreen",
		fragment: Some("blit"),
		primitive: default(),
		depth_stencil: None,
		multisample: default()
	}).unwrap();
}
