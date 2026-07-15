pub mod util;

mod surface;

pub(crate) use surface::*;

use thiserror::Error;
use wgpu::{
    Adapter, CommandEncoder, CommandEncoderDescriptor, Device, Instance, Queue,
    RequestAdapterError, RequestDeviceError
};

use crate::RendererConfig;


pub(crate) struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue
}

impl GpuContext {
    pub async fn try_new(config: RendererConfig<'_>) -> Result<Self, GpuContextError> {
        let instance = Instance::new(config.instance_descriptor());

        let adapter = instance
            .request_adapter(&config.request_adapter_options())
            .await?;

        let (device, queue) = adapter.request_device(&config.device_descriptor()).await?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue
        })
    }

    pub fn create_encoder(&self, label: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: Some(label) })
    }
}


#[derive(Error, Debug)]
pub enum GpuContextError {
    #[error("Adapter not found")]
    AdapterNotFound,
    #[error(transparent)]
    FailedToRequestDevice(#[from] RequestDeviceError)
}

impl From<RequestAdapterError> for GpuContextError {
    fn from(_err: RequestAdapterError) -> Self { Self::AdapterNotFound }
}
