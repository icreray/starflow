mod frame;
mod surface;
pub mod util;

pub(crate) use frame::*;
pub(crate) use surface::*;

use wgpu::{Adapter, CommandEncoder, CommandEncoderDescriptor, Device, Instance, Queue};

use crate::RendererConfig;


pub(crate) struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue
}

impl GpuContext {
    pub async fn new(config: RendererConfig<'_>) -> Self {
        let instance = Instance::new(config.instance_descriptor());

        let adapter = instance
            .request_adapter(&config.request_adapter_options())
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(&config.device_descriptor())
            .await
            .expect("Failed to request device");

        Self {
            instance,
            adapter,
            device,
            queue
        }
    }

    pub fn create_encoder(&self, label: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: Some(label) })
    }
}
