// alloc_mem: VirtualAlloc实现
#[cfg(feature = "alloc_mem_va")]
mod va;
#[cfg(feature = "alloc_mem_va")]
pub use crate::alloc_mem::va::alloc_mem;

// alloc_mem: GlobalAlloc实现
#[cfg(feature = "alloc_mem_global")]
mod global;
#[cfg(feature = "alloc_mem_global")]
pub use crate::alloc_mem::global::alloc_mem;

// alloc_mem: LocalAlloc实现
#[cfg(feature = "alloc_mem_local")]
mod local;
#[cfg(feature = "alloc_mem_local")]
pub use crate::alloc_mem::local::alloc_mem;