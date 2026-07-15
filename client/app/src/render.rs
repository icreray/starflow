use futures_lite::future;

use starflow_render::{
    Renderer,
    assets::{AssetError, AssetResult, RenderAssets, RenderAssetsCreation},
    config::{Features, RendererConfig},
    graph::{
        ComputePassDescriptor, ComputePipeline, FrameContext, RenderNode,
        RenderPassDescriptor, RenderPipeline
    },
    resources::RenderResources,
    util::Color
};
use starflow_util::{Handle, default};
use starflow_window::WindowModule;


pub fn create_renderer<'w>(window: &WindowModule) -> Renderer<'w> {
    let context_config = RendererConfig::default()
        .add_features(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
    let mut renderer =
        future::block_on(Renderer::try_new(context_config, window.clone_handle()))
            .expect("Failed to initialize renderer");
    renderer.create_assets(create_assets);
    renderer.set_graph(|ctx| {
        ctx.add_node::<MainPass>()
            .expect("Failed to init main pass");
        ctx.add_node::<BlitPass>()
            .expect("Failed to init blit pass");
    });
    renderer
}


fn create_assets(ctx: &mut RenderAssetsCreation) {
    use starflow_render::assets::{desc::*, util::*};

    ctx.create(BindGroupLayout::new("output_texture", &[binding(0)
        .visibility(ShaderStages::COMPUTE)
        .texture_storage_2d(
            TextureFormat::Rgba8Unorm,
            StorageTextureAccess::WriteOnly
        )]))
    .unwrap();

    ctx.create(BindGroupLayout::new("input_texture", &[binding(0)
        .visibility(ShaderStages::FRAGMENT)
        .texture_storage_2d(
            TextureFormat::Rgba8Unorm,
            StorageTextureAccess::ReadOnly
        )]))
    .unwrap();

    // main pass
    ctx.create(PipelineLayout {
        key: "main_pass",
        bind_group_layouts: &[Some("output_texture")],
        immediate_size: 0
    })
    .unwrap();
    ctx.create(ShaderModule::new(
        "main_pass",
        ShaderSource::Wgsl(include_str!("../../../assets/shaders/main_pass.wgsl").into())
    ))
    .unwrap();

    ctx.create(ComputePipeline {
        key: "main_pass",
        layout: Some("main_pass"),
        module: "main_pass"
    })
    .unwrap();

    // blit
    ctx.create(ShaderModule::new(
        "fullscreen",
        ShaderSource::Wgsl(
            include_str!("../../../assets/shaders/fullscreen.wgsl").into()
        )
    ))
    .unwrap();

    ctx.create(ShaderModule::new(
        "blit",
        ShaderSource::Wgsl(include_str!("../../../assets/shaders/blit.wgsl").into())
    ))
    .unwrap();

    ctx.create(PipelineLayout {
        key: "blit",
        bind_group_layouts: &[Some("input_texture")],
        immediate_size: 0
    })
    .unwrap();

    ctx.create(RenderPipeline {
        key: "blit",
        layout: Some("blit"),
        vertex: "fullscreen",
        fragment: Some("blit"),
        primitive: default(),
        depth_stencil: None,
        multisample: default()
    })
    .unwrap();
}


pub struct MainPass {
    main_pass: Handle<ComputePipeline>
}

impl<'a> TryFrom<&'a RenderAssets> for MainPass {
    type Error = AssetError<'a>;

    fn try_from(assets: &'a RenderAssets) -> AssetResult<'a, Self> {
        let main_pass = assets.get_dependency_handle("main_pass")?;
        Ok(Self { main_pass })
    }
}

impl RenderNode for MainPass {
    fn run(
        &self,
        frame: &mut FrameContext,
        assets: &RenderAssets,
        resources: &RenderResources
    ) {
        let mut pass = frame.encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("main_pass"),
            timestamp_writes: None
        });
        pass.set_pipeline(&assets[&self.main_pass]);
        pass.set_bind_group(0, &resources.output_texture_bind_group, &[]);
        pass.dispatch_workgroups(
            (frame.texture.width() + 15) >> 4,
            (frame.texture.height() + 15) >> 4,
            1
        );
    }
}


pub struct BlitPass {
    blit: Handle<RenderPipeline>
}

impl<'a> TryFrom<&'a RenderAssets> for BlitPass {
    type Error = AssetError<'a>;

    fn try_from(assets: &'a RenderAssets) -> AssetResult<'a, Self> {
        let blit = assets.get_dependency_handle("blit")?;
        Ok(Self { blit })
    }
}

impl RenderNode for BlitPass {
    fn run(
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
