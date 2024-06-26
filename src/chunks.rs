use std::{
    any::{Any, TypeId},
    cell::RefMut,
    collections::HashMap,
};

use anyhow::Result;
use ratatui::prelude::Rect;

use crate::*;

/// The default system of storage for the system.
#[derive(Default, State)]
pub struct Chunks {
    chunks: HashMap<TypeId, Rect>,
}

impl Chunks {
    /// Clears the chunks from the state.
    pub fn clear(&mut self) {
        self.chunks = HashMap::new();
    }

    /// Register a chunk to the state, with the key being a type id.
    pub fn register_chunk<T: Any>(&mut self, rect: Rect) {
        self.chunks.insert(TypeId::of::<T>(), rect);
    }

    /// Returns a rect if the type id is within the chunk,
    /// an error is thrown if it isn't registered.
    pub fn get_chunk<T: Any>(&self) -> Result<Rect> {
        match self.chunks.get(&TypeId::of::<T>()).cloned() {
            Some(chunk) => Ok(chunk),
            None => bail!("Chunk doesn't exist"),
        }
    }
}
