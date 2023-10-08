use num::Float;

use crate::{
    Code,
    SumRepr,
    Terms,
};

#[derive(Debug)]
pub struct IterRepr<T, K, I>
where
    I: Iterator<Item = (T, K)>,
{
    iter: I,
}

impl<T, K, I> IterRepr<T, K, I>
where
    I: Iterator<Item = (T, K)>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
        }
    }
}

impl<T, K, I> Terms<T, K> for IterRepr<T, K, I>
where
    T: Float,
    K: Code,
    I: Iterator<Item = (T, K)>,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        for (coeff, code) in self.iter.by_ref() {
            repr.add(code, coeff);
        }
    }
}

#[derive(Debug)]
pub struct OpRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    f: OP,
}

impl<T, K, OP> OpRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    pub fn new(f: OP) -> Self {
        Self {
            f,
        }
    }
}

impl<T, K, OP> Terms<T, K> for OpRepr<T, K, OP>
where
    T: Float,
    K: Code,
    OP: FnMut() -> Option<(T, K)>,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        while let Some((coeff, code)) = (self.f)() {
            repr.add(code, coeff);
        }
    }
}

pub struct HeapRepr<'a, T, K> {
    f: Box<dyn FnMut() -> Option<(T, K)> + 'a>,
}

impl<'a, T, K> HeapRepr<'a, T, K> {
    /// Allocate memory for the closure on the heap.
    pub fn new<OP>(f: OP) -> Self
    where
        OP: FnMut() -> Option<(T, K)> + 'a,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<'a, T, K> Terms<T, K> for HeapRepr<'a, T, K>
where
    T: Float,
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        while let Some((coeff, code)) = (self.f)() {
            repr.add(code, coeff);
        }
    }
}
