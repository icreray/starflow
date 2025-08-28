use std::{marker::PhantomData, ops::Index};

use ahash::AHashMap;


pub(crate) struct ResourceCache<R> {
	key_to_id: AHashMap<&'static str, ResourceId<R>>,
	resources: Vec<R>
}

impl<R> Default for ResourceCache<R> {
	fn default() -> Self {
		Self {
			key_to_id: Default::default(),
			resources: Default::default()
		}
	}
}

#[allow(dead_code)]
impl<R> ResourceCache<R> {
	pub fn add(&mut self, key: &'static str, resource: R) -> Option<ResourceId<R>> {
		if self.key_to_id.contains_key(key) {
			None
		}
		else {
			Some(self.add_unchecked(key, resource))
		}
	}

	pub fn add_unchecked(&mut self, key: &'static str, resource: R) -> ResourceId<R> {
		let id = ResourceId(self.resources.len(), PhantomData);
		self.key_to_id.insert(key, id.clone());
		self.resources.push(resource);
		id
	}

	pub fn contains_key(&self, key: &str) -> bool {
		self.key_to_id.contains_key(key)
	}

	pub fn get(&self, key: &str) -> Option<&R> {
		self.get_id(key).map(|id| &self[id])
	}

	pub fn get_id(&self, key: &str) -> Option<ResourceId<R>> {
		self.key_to_id.get(key).cloned()
	}
}

impl<R> Index<ResourceId<R>> for ResourceCache<R> {
	type Output = R;

	fn index(&self, index: ResourceId<R>) -> &Self::Output {
		&self.resources[index.0]
	}
}


#[derive(Copy)]
pub(crate) struct ResourceId<R>(usize, PhantomData<R>);

impl<R> Clone for ResourceId<R> {
	fn clone(&self) -> Self {
		Self(self.0, self.1)
	}
}
