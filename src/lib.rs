#![feature(collections, raw, get_type_id)]
#![no_std]


extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate scene_graph;
extern crate shared;


mod scene_renderer;
mod renderer;


pub use scene_renderer::SceneRenderer;
pub use renderer::Renderer;
