#![feature(alloc)]
#![feature(collections)]
#![feature(get_type_id)]
#![feature(raw)]
#![no_std]


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate shared;
extern crate scene_graph;
extern crate hash_map;
extern crate vector;
extern crate map;
extern crate iterable;
extern crate iterable_mut;
extern crate stack;
extern crate insert;
extern crate remove;


mod scene_renderer;
mod renderer;


pub use scene_renderer::SceneRenderer;
pub use renderer::Renderer;
