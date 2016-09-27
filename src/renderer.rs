use core::any::Any;

use scene_graph::Id;

use scene_renderer::SceneRenderer;


pub trait Renderer: Any {

    fn get_id(&self) -> Id;

    fn get_scene_renderer(&self) -> Option<SceneRenderer>;
    fn set_scene_renderer(&self, renderer: Option<SceneRenderer>);

    fn get_order(&self) -> usize;

    fn before_render(&self);
    fn after_render(&self);
    fn render(&self);
}

impl Renderer {
    impl_any!();
}
