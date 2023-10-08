//! Representation of Hamiltonian sum terms

#![allow(dead_code)]

use std::collections::HashMap;

use num::Float;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Code,
    Terms,
};

mod conversions;
mod representations;

/// Weighted sum of codes
#[derive(Debug, Serialize, Deserialize)]
pub struct SumRepr<T, K>
where
    K: Code,
{
    map: HashMap<K, T>,
}

impl<T, K> Default for SumRepr<T, K>
where
    K: Code,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, K> SumRepr<T, K>
where
    K: Code,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn as_map(&self) -> &HashMap<K, T> {
        &self.map
    }

    pub fn as_map_mut(&mut self) -> &mut HashMap<K, T> {
        &mut self.map
    }
}

impl<T, K> SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    #[must_use]
    pub fn coeff(
        &self,
        code: &K,
    ) -> T {
        match self.map.get(code) {
            Some(coeff) => *coeff,
            None => T::zero(),
        }
    }

    pub fn update(
        &mut self,
        code: K,
        coeff: T,
    ) -> Option<T> {
        self.map.insert(code, coeff)
    }

    pub fn add(
        &mut self,
        code: K,
        coeff: T,
    ) {
        let prev_coeff = self.coeff(&code);
        let _ = self.update(code, coeff + prev_coeff);
    }
}

impl<T, K> Terms<T, K> for SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        for (code, value) in self.as_map() {
            repr.add(code.clone(), *value);
        }
    }
}
