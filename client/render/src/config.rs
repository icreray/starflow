pub use wgpu::{
    Backends, Features, InstanceFlags, Label, Limits, MemoryHints, PowerPreference
};
use wgpu::{
    DeviceDescriptor, ExperimentalFeatures, InstanceDescriptor, RequestAdapterOptions,
    Trace
};


pub struct RendererConfig<'label> {
    pub backends: Backends,
    pub power_preference: PowerPreference,
    pub device_label: Label<'label>,
    pub required_features: Features,
    pub required_limits: Limits,
    pub memory_hints: MemoryHints
}

impl Default for RendererConfig<'_> {
    fn default() -> Self {
        Self {
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
impl<'l> RendererConfig<'l> {
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

// Helper functions
impl RendererConfig<'_> {
    pub(crate) fn instance_descriptor(&self) -> InstanceDescriptor {
        InstanceDescriptor {
            backends: self.backends,
            ..InstanceDescriptor::new_without_display_handle()
        }
    }

    pub(crate) fn request_adapter_options<'s>(&self) -> RequestAdapterOptions<'_, 's> {
        RequestAdapterOptions {
            power_preference: self.power_preference,
            force_fallback_adapter: false,
            compatible_surface: None
        }
    }

    pub(crate) fn device_descriptor(&self) -> DeviceDescriptor<'_> {
        DeviceDescriptor {
            label: self.device_label,
            required_features: self.required_features,
            required_limits: self.required_limits.clone(),
            experimental_features: ExperimentalFeatures::disabled(),
            memory_hints: self.memory_hints.clone(),
            trace: Trace::Off
        }
    }
}
