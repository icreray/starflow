mod surface;

pub(crate) use surface::*;

use wgpu::{Adapter, Device, Instance, Queue};

use crate::GpuContextConfig;

pub(crate) struct GpuContext {
	pub instance: Instance,
	pub adapter: Adapter,
	pub device: Device,
	pub queue: Queue
}

impl GpuContext {
	pub async fn new(config: GpuContextConfig<'_>) -> Self {
		let instance = Instance::new(&config.instance_descriptor());

		let adapter = instance.request_adapter(&config.request_adapter_options())
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = adapter
			.request_device(&config.device_descriptor())
			.await
			.expect("Failed to request device");

		Self {
			instance, adapter, device, queue
		}
	}
}
