#[cfg(truffleruby)]
pub use truffleruby_sys::*;

#[cfg(truffleruby)]
pub fn rb_gc_adjust_memory_usage(_: isize) {}

#[cfg(truffleruby)]
pub struct RData;
