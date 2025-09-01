use default::default;

pub use wgpu::{PushConstantRange, ShaderSource, PrimitiveState, DepthStencilState, MultisampleState};
use wgpu::{
	BindGroupLayoutDescriptor, BindGroupLayoutEntry, BlendState, ColorTargetState, ColorWrites,
	ComputePipelineDescriptor, FragmentState, PipelineLayoutDescriptor, RenderPipelineDescriptor,
	ShaderModuleDescriptor, VertexState
};

use crate::assets::AssetResult;

use super::{RenderAssetDesc, RenderAssetsCreation};


pub struct BindGroupLayout<'a> {
	pub key: &'a str,
	pub entries: &'a [BindGroupLayoutEntry]
}

impl<'a> BindGroupLayout<'a> {
	pub fn new(
		key: &'a str,
		entries: &'a [BindGroupLayoutEntry]
	) -> Self {
		Self { key, entries }
	}
}

impl<'a> RenderAssetDesc<'a> for BindGroupLayout<'a> {
	type Asset = wgpu::BindGroupLayout;

	fn key(&self) -> &str { &self.key }

	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset> {
		Ok(ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some(self.key),
			entries: self.entries
		}))
	}
}


pub struct PipelineLayout<'a> {
	pub key: &'a str,
	pub bind_group_layouts: &'a [&'a str],
	pub push_constant_ranges: &'a [PushConstantRange]
}

impl<'a> RenderAssetDesc<'a> for PipelineLayout<'a> {
	type Asset = wgpu::PipelineLayout;

	fn key(&self) -> &str { &self.key }

	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset> {
		let layouts = self.bind_group_layouts.iter()
			.map(|&layout| ctx.get_dependency_asset(layout))
			.collect::<AssetResult<'a, Vec<_>>>()?;
		Ok(ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some(self.key),
			bind_group_layouts: &layouts,
			push_constant_ranges: self.push_constant_ranges
		}))
	}
}


pub struct ShaderModule<'a> {
	pub key: &'a str,
	pub source: ShaderSource<'a>
}

impl<'a> ShaderModule<'a> {
	pub fn new(key: &'a str, source: ShaderSource<'a>) -> Self {
		Self { key, source }
	}
}

impl<'a> RenderAssetDesc<'a> for ShaderModule<'a> {
	type Asset = wgpu::ShaderModule;

	fn key(&self) -> &str { &self.key }

	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset> {
		Ok(ctx.device.create_shader_module(ShaderModuleDescriptor {
			label: Some(self.key),
			source: self.source
		}))
	}
}


pub struct ComputePipeline<'a> {
	pub key: &'a str,
	pub layout: Option<&'a str>,
	pub module: &'a str
}

impl<'a> RenderAssetDesc<'a> for ComputePipeline<'a> {
	type Asset = wgpu::ComputePipeline;

	fn key(&self) -> &str { &self.key }

	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset> {
		let layout = self.layout
			.map(|layout| ctx.get_dependency_asset(layout))
			.transpose()?;
		let module = ctx.get_dependency_asset(self.module)?;

		Ok(ctx.device.create_compute_pipeline(&ComputePipelineDescriptor {
			label: Some(self.key),
			layout,
			module,
			entry_point: None,
			// TODO: More granular control (if needed, otherwise remove this todo in future :))
			compilation_options: default(),
			cache: None
		}))
	}
}


// TODO: This is proof of concept. Add more granular control over shaders and other parameters
pub struct RenderPipeline<'a> {
	pub key: &'a str,
	pub layout: Option<&'a str>,
	pub vertex: &'a str,
	pub fragment: Option<&'a str>,
	pub primitive: PrimitiveState,
	pub depth_stencil: Option<DepthStencilState>,
	pub multisample: MultisampleState
}

impl<'a> RenderAssetDesc<'a> for RenderPipeline<'a> {
	type Asset = wgpu::RenderPipeline;

	fn key(&self) -> &str { &self.key }

	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset> {
		let layout = self.layout
			.map(|layout| ctx.get_dependency_asset(layout))
			.transpose()?;

		let vertex = ctx.get_dependency_asset(self.vertex)?;

		let fragment = match self.fragment {
			Some(fragment) => {
				let fragment = ctx.get_dependency_asset(fragment)?;
				Some(FragmentState {
					module: fragment,
					entry_point: None,
					compilation_options: default(),
					targets: &[Some(ColorTargetState {
						format: ctx.surface.texture_format(),
						blend: Some(BlendState::REPLACE),
						write_mask: ColorWrites::ALL,
					})]
				})
			}
			None => None
		};

		Ok(ctx.device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some(self.key),
			layout,
			vertex: VertexState {
				module: vertex,
				entry_point: None,
				compilation_options: default(),
				buffers: &[]
			},
			fragment,
			primitive: self.primitive,
			depth_stencil: self.depth_stencil,
			multisample: self.multisample,
			multiview: None,
			cache: None
		}))
	}
}
