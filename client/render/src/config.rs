use default::default;

pub use wgpu::{Backends, Features, InstanceFlags, Label, Limits, MemoryHints, PowerPreference};
use wgpu::{DeviceDescriptor, InstanceDescriptor, RequestAdapterOptions, Trace};


pub struct GpuContextConfig<'label> {
	pub instance_flags: InstanceFlags,
	pub backends: Backends,
	pub power_preference: PowerPreference,
	pub device_label: Label<'label>,
	pub required_features: Features,
	pub required_limits: Limits,
	pub memory_hints: MemoryHints
}

impl Default for GpuContextConfig<'_> {
	fn default() -> Self {
		Self {
			instance_flags: InstanceFlags::default(),
			backends: Backends::VULKAN,
			power_preference: PowerPreference::HighPerformance,
			device_label: None,
			required_features: Features::empty(),
			required_limits: Limits::default(),
			memory_hints: MemoryHints::Performance
		}
	}
}

// Chaining mutations
impl<'l> GpuContextConfig<'l> {
	pub fn add_flags(mut self, flags: InstanceFlags) -> Self {
		self.instance_flags |= flags;
		self
	}

	pub fn flags(mut self, flags: InstanceFlags) -> Self {
		self.instance_flags = flags;
		self
	}

	pub fn add_backends(mut self, backends: Backends) -> Self {
		self.backends |= backends;
		self
	}

	pub fn backends(mut self, backends: Backends) -> Self {
		self.backends = backends;
		self
	}

	pub fn power_preference(mut self, power_preference: PowerPreference) -> Self {
		self.power_preference = power_preference;
		self
	}

	pub fn device_label(mut self, label: &'l str) -> Self {
		self.device_label = Some(label);
		self
	}

	pub fn add_features(mut self, features: Features) -> Self {
		self.required_features |= features;
		self
	}

	pub fn features(mut self, features: Features) -> Self {
		self.required_features = features;
		self
	}

	pub fn memory_hints(mut self, memory_hints: MemoryHints) -> Self {
		self.memory_hints = memory_hints;
		self
	}
}

impl GpuContextConfig<'_> {
	pub(crate) fn instance_descriptor(&self) -> InstanceDescriptor {
		InstanceDescriptor {
			backends: self.backends,
			flags: self.instance_flags,
			..default()
		}
	}

	pub(crate) fn request_adapter_options(&self) -> RequestAdapterOptions {
		RequestAdapterOptions {
			power_preference: self.power_preference,
			force_fallback_adapter: false,
			compatible_surface: None
		}
	}

	pub(crate) fn device_descriptor(&self) -> DeviceDescriptor {
		DeviceDescriptor {
			label: self.device_label,
			required_features: self.required_features,
			required_limits: self.required_limits.clone(),
			memory_hints: self.memory_hints.clone(),
			trace: Trace::Off,
		}
	}
}
