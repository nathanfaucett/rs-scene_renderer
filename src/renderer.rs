use core::any::Any;

use scene_graph::Id;

use scene_renderer::SceneRenderer;


pub trait Renderer: Any {

    fn get_id(&self) -> Id;

    fn get_scene_renderer(&self) -> Option<SceneRenderer>;
    fn set_scene_renderer(&mut self, renderer: Option<SceneRenderer>);

    fn get_order(&self) -> usize;

    fn before_render(&mut self);
    fn after_render(&mut self);
    fn render(&mut self);
}

impl Renderer {
    impl_any!();
}
