#![no_std]
#![feature(collections, alloc)]


extern crate alloc;
extern crate collections;

extern crate scene_graph;
extern crate scene_renderer;

use alloc::rc::Rc;
use core::cell::RefCell;

use scene_graph::{Id, Scene};

use scene_renderer::{SceneRenderer, Renderer};


struct SomeRendererData {
    scene_renderer: Option<SceneRenderer>,
}
#[derive(Clone)]
pub struct SomeRenderer {
    data: Rc<RefCell<SomeRendererData>>,
}
impl SomeRenderer {
    pub fn new() -> Self {
        SomeRenderer {
            data: Rc::new(RefCell::new(SomeRendererData {
                scene_renderer: None,
            }))
        }
    }
}
impl Renderer for SomeRenderer {

    fn get_id(&self) -> Id { Id::of::<SomeRenderer>() }

    fn get_scene_renderer(&self) -> Option<SceneRenderer> {
        self.data.borrow().scene_renderer.clone()
    }
    fn set_scene_renderer(&mut self, renderer: Option<SceneRenderer>) {
        self.data.borrow_mut().scene_renderer = renderer;
    }

    fn get_order(&self) -> usize { 0 }

    fn before_render(&mut self) {}
    fn after_render(&mut self) {}
    fn render(&mut self) {}
}
impl PartialEq<SomeRenderer> for SomeRenderer {
    fn eq(&self, other: &SomeRenderer) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
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
