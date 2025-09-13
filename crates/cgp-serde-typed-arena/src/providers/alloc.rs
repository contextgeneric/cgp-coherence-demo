use cgp::prelude::*;
use cgp_serde_alloc::traits::{Allocator, AllocatorComponent};

use crate::traits::HasArena;

#[cgp_new_provider]
impl<'a, Context, Value: 'a> Allocator<'a, Context, Value> for ArenaAllocator
where
    Context: HasArena<'a, Value>,
{
    fn alloc(context: &Context, value: Value) -> &'a mut Value {
        context.arena().alloc(value)
    }
}
