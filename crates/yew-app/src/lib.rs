pub mod app;
pub mod components;
pub mod routes;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;