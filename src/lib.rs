#![feature(alloc)]
#![feature(collections)]
#![feature(get_type_id)]
#![feature(raw)]
#![no_std]


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate scene_graph;
extern crate shared;


mod scene_renderer;
mod renderer;


pub use scene_renderer::SceneRenderer;
pub use renderer::Renderer;
