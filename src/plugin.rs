use core::any::Any;

use scene_graph::Id;

use scene_renderer::SceneRenderer;


pub trait Plugin: Any {

    fn get_id(&self) -> Id;

    fn get_scene_renderer(&self) -> Option<SceneRenderer>;
    fn set_scene_renderer(&mut self, scene: Option<SceneRenderer>);

    fn get_order(&self) -> usize;

    fn clear(&mut self);
    fn init(&mut self);
    fn before_render(&mut self);
    fn after_render(&mut self);
}

impl Plugin {
    impl_any!();
}
