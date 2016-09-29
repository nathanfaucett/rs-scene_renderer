use collections::boxed::Box;
use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;

use scene_graph::{Id, Scene};

use renderer::Renderer;


struct SceneRendererData {
    scene: Scene,
    renderers: Vec<Rc<RefCell<Box<Renderer>>>>,
    renderers_map: BTreeMap<Id, Rc<RefCell<Box<Renderer>>>>
}

#[derive(Clone)]
pub struct SceneRenderer {
    data: Rc<RefCell<SceneRendererData>>,
}

impl SceneRenderer {

    pub fn new(scene: Scene) -> Self {
        SceneRenderer {
            data: Rc::new(RefCell::new(SceneRendererData {
                scene: scene,
                renderers: Vec::new(),
                renderers_map: BTreeMap::new(),
            }))
        }
    }

    pub fn scene(&self) -> Scene {
        self.data.borrow().scene.clone()
    }

    pub fn render(&self) -> &Self {
        for renderer in self.data.borrow().renderers.iter() {
            renderer.borrow().render();
        }
        self
    }

    pub fn add_renderer<T: Renderer + Clone>(&self, renderer: T) -> &Self {
        let id = Id::of::<T>();

        if !self.data.borrow().renderers_map.contains_key(&id) {
            renderer.set_scene_renderer(Some(self.clone()));
            let renderer_wrap = Rc::new(RefCell::new(Box::new(renderer) as Box<Renderer>));
            self.data.borrow_mut().renderers.push(renderer_wrap.clone());
            self.data.borrow_mut().renderers_map.insert(id, renderer_wrap);
            self.sort_renderers();
        }
        self
    }
    pub fn has_renderer<T: Renderer + Clone>(&self) -> bool {
        self.data.borrow().renderers_map.contains_key(&Id::of::<T>())
    }
    pub fn remove_renderer<T: Renderer + Clone>(&self) -> &Self {
        let id = Id::of::<T>();

        if self.data.borrow().renderers_map.contains_key(&id) {
            {
                let ref mut renderers = self.data.borrow_mut().renderers;
                match renderers.iter().position(|c| c.borrow().id() == id) {
                    Some(i) => {
                        renderers[i].borrow_mut().set_scene_renderer(None);
                        renderers.remove(i);
                    },
                    None => (),
                }
            }
            self.data.borrow_mut().renderers_map.remove(&id);
        }
        self
    }
    pub fn get_renderer<T: Renderer + Clone>(&self) -> Option<T> {
        let ref renderers_map = self.data.borrow().renderers_map;
        let id = Id::of::<T>();

        if renderers_map.contains_key(&id) {
            let renderer_ref = renderers_map.get(&id).unwrap().borrow();
            let renderer = renderer_ref.downcast_ref::<T>().unwrap();
            Some(renderer.clone())
        } else {
            None
        }
    }

    fn sort_renderers(&self) {
        self.data.borrow_mut().renderers.sort_by(|a, b| {
            a.borrow().order().cmp(&b.borrow().order())
        });
    }
}
