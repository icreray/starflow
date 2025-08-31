use default::default;

use wgpu::{Color, ComputePassDescriptor, ComputePipeline, RenderPassDescriptor, RenderPipeline};

use starflow_util::Handle;

use crate::{
	assets::RenderAssets,
	core::FrameContext,
	resources::RenderResources
};


pub(crate) struct RenderGraph {
	main_pass: Handle<ComputePipeline>,
	blit: Handle<RenderPipeline>
}

impl RenderGraph {
	pub(crate) fn new(assets: &RenderAssets) -> Self {
		Self {
			main_pass: assets.get_handle("main_pass").unwrap(),
			blit: assets.get_handle("blit").unwrap()
		}
	}
}

impl RenderGraph {
	pub fn run(
		&self,
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		self.run_main_pass(frame, assets, resources);
		self.run_blit_pass(frame, assets, resources);
	}

	fn run_main_pass(
		&self,
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let mut pass = frame.encoder.begin_compute_pass(&ComputePassDescriptor {
			label: Some("main_pass"),
			timestamp_writes: None,
		});
		pass.set_pipeline(&assets[&self.main_pass]);
		pass.set_bind_group(0, &resources.output_texture_bind_group, &[]);
		pass.dispatch_workgroups(
			(frame.texture.width() + 15) >> 4,
			(frame.texture.height() + 15) >> 4,
			1
		);
	}

	fn run_blit_pass(
		&self,
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let attachment = frame.texture.clear_attachment(Color::BLACK);
		let mut pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("display"),
			color_attachments: &[Some(attachment)],
			..default()
		});
		pass.set_pipeline(&assets[&self.blit]);
		pass.set_bind_group(0, &resources.input_texture_bind_group, &[]);
		pass.draw(0..3, 0..1);
	}
}
