use std::{borrow::Borrow, hash::Hash, marker::PhantomData, ops::Index};
use default::default;

use ahash::AHashMap;


pub struct Registry<K, V> {
	key_to_handle: AHashMap<K, Handle<V>>,
	values: Vec<V>
}

impl<K, V> Default for Registry<K, V> {
	fn default() -> Self {
		Self {
			key_to_handle: default(),
			values: default()
		}
	}
}


impl<K, V> Registry<K, V>
where K: Eq + Hash  {

	pub fn set(&mut self, key: K, value: V) -> Handle<V> {
		if let Some(handle) = self.key_to_handle.get(&key) {
			self.values[handle.0] = value;
			handle.clone()
		}
		else {
			let handle = Handle::new(self.values.len());
			self.key_to_handle.insert(key, handle.clone());
			self.values.push(value);
			handle
		}
	}

	pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq
	{
		self.get_handle(key).map(|id| &self[id])
	}

	pub fn get_handle<Q: ?Sized>(&self, key: &Q) -> Option<Handle<V>>
	where
		K: Borrow<Q>,
		Q: Hash + Eq
	{
		self.key_to_handle.get(key).cloned()
	}
}

impl<K, V> Index<Handle<V>> for Registry<K, V> {
	type Output = V;

	fn index(&self, index: Handle<V>) -> &Self::Output {
		&self.values[index.0]
	}
}


#[derive(Copy)]
pub struct Handle<V>(usize, PhantomData<V>);

impl<V> Clone for Handle<V> {
	fn clone(&self) -> Self {
		Self(self.0, self.1)
	}
}

impl<V> Handle<V> {
	fn new(id: usize) -> Self {
		Self(id, PhantomData)
	}
}
