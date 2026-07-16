mod bind_groups;
mod surface;

pub(crate) use bind_groups::*;
pub use surface::*;

pub use wgpu::Color;


pub(crate) type Key = Box<str>;
