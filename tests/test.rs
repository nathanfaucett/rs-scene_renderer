#![feature(alloc)]
#![feature(collections)]
#![no_std]


extern crate alloc;
extern crate collections;

extern crate shared;
extern crate scene_graph;

extern crate scene_renderer;

use shared::Shared;
use scene_graph::{Id, Scene};

use scene_renderer::{SceneRenderer, Renderer};


struct SomeRendererData {
    scene_renderer: Option<SceneRenderer>,
}
#[derive(Clone)]
pub struct SomeRenderer {
    data: Shared<SomeRendererData>,
}
impl SomeRenderer {
    pub fn new() -> Self {
        SomeRenderer {
            data: Shared::new(SomeRendererData {
                scene_renderer: None,
            })
        }
    }
}
impl Renderer for SomeRenderer {

    fn get_id(&self) -> Id { Id::of::<SomeRenderer>() }

    fn get_scene_renderer(&self) -> Option<SceneRenderer> {
        self.data.scene_renderer.clone()
    }
    fn set_scene_renderer(&mut self, renderer: Option<SceneRenderer>) {
        self.data.scene_renderer = renderer;
    }

    fn get_order(&self) -> usize { 0 }

    fn before_render(&mut self) {}
    fn after_render(&mut self) {}
    fn render(&mut self) {}
}
impl PartialEq<SomeRenderer> for SomeRenderer {
    fn eq(&self, other: &SomeRenderer) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &SomeRenderer) -> bool {
        !self.eq(other)
    }
}


#[test]
fn test_scene() {
    let scene = Scene::new();
    let mut scene_renderer = SceneRenderer::new(scene.clone());
    let some_renderer = SomeRenderer::new();

    scene_renderer.add_renderer(some_renderer.clone());
    assert_eq!(scene_renderer.has_renderer::<SomeRenderer>(), true);

    let renderer = scene_renderer.get_renderer::<SomeRenderer>().unwrap();
    assert_eq!(renderer == some_renderer, true);

    scene_renderer.render();

    scene_renderer.remove_renderer::<SomeRenderer>();
    assert_eq!(scene_renderer.has_renderer::<SomeRenderer>(), false);
}
