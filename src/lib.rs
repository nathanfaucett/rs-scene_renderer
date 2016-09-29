#![feature(collections, alloc, raw, get_type_id)]
#![no_std]


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate scene_graph;


mod scene_renderer;
mod renderer;


pub use scene_renderer::SceneRenderer;
pub use renderer::Renderer;
