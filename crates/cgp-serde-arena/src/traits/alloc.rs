use cgp::prelude::*;

#[cgp_component(Allocator)]
pub trait CanAlloc<'a, T> {
    fn alloc(&self, value: T) -> &'a mut T;
}
