use default::default;

use wgpu::{Color, ComputePassDescriptor, RenderPassDescriptor};

use crate::{core::FrameContext, resources::RenderResources};


pub(crate) struct RenderGraph;

impl RenderGraph {
	pub fn run(
		frame: &mut FrameContext,
		resources: &RenderResources
	) {
		Self::run_main_pass(frame, resources);
		Self::run_blit_pass(frame, resources);
	}

	fn run_main_pass(
		frame: &mut FrameContext,
		resources: &RenderResources
	) {
		let mut pass = frame.encoder.begin_compute_pass(&ComputePassDescriptor {
			label: Some("main_pass"),
			timestamp_writes: None,
		});
		pass.set_pipeline(&resources.pipelines.main_pass);
		pass.set_bind_group(0, &resources.allocated.output_texture_bind_group, &[]);
		pass.dispatch_workgroups(
			(frame.texture.width() + 15) >> 4,
			(frame.texture.height() + 15) >> 4,
			1
		);
	}

	fn run_blit_pass(
		frame: &mut FrameContext,
		resources: &RenderResources
	) {
		let attachment = frame.texture.clear_attachment(Color::BLACK);
		let mut pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("display"),
			color_attachments: &[Some(attachment)],
			..default()
		});
		pass.set_pipeline(&resources.pipelines.blit);
		pass.set_bind_group(0, &resources.allocated.input_texture_bind_group, &[]);
		pass.draw(0..3, 0..1);
	}
}
