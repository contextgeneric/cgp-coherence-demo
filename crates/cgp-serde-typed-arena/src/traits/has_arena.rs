use cgp::prelude::*;
use typed_arena::Arena;

#[cgp_getter]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}
