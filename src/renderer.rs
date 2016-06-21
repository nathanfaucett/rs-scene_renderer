use core::any::Any;
use core::any::TypeId;
use core::raw::TraitObject;
use core::mem::transmute;

use scene_graph::Id;

use scene_renderer::SceneRenderer;


pub trait Renderer: Any {

    fn id(&self) -> Id;

    fn scene_renderer(&self) -> Option<SceneRenderer>;
    fn set_scene_renderer(&self, renderer: Option<SceneRenderer>);

    fn order(&self) -> usize;

    fn init(&self);
    fn destroy(&self);

    fn before_render(&self);
    fn after_render(&self);
    fn render(&self);
}

impl Renderer {
    pub fn is<T: Any>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = self.get_type_id();
        t == boxed
    }
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                let to: TraitObject = transmute(self);
                Some(&*(to.data as *const T))
            }
        } else {
            None
        }
    }
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                let to: TraitObject = transmute(self);
                Some(&mut *(to.data as *const T as *mut T))
            }
        } else {
            None
        }
    }
}
