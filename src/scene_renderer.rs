use alloc::boxed::Box;
use collections::vec::Vec;
use collections::btree_map::BTreeMap;

use shared::Shared;
use scene_graph::{Id, Scene};

use renderer::Renderer;


struct SceneRendererData {
    scene: Scene,
    renderers: Vec<Shared<Box<Renderer>>>,
    renderers_map: BTreeMap<Id, Shared<Box<Renderer>>>
}

#[derive(Clone)]
pub struct SceneRenderer {
    data: Shared<SceneRendererData>,
}

impl SceneRenderer {

    pub fn new(scene: Scene) -> Self {
        SceneRenderer {
            data: Shared::new(SceneRendererData {
                scene: scene,
                renderers: Vec::new(),
                renderers_map: BTreeMap::new(),
            })
        }
    }

    pub fn get_scene(&self) -> Scene {
        self.data.scene.clone()
    }

    pub fn render(&mut self) -> &Self {
        for renderer in self.data.renderers.iter_mut() {
            renderer.render();
        }
        self
    }

    pub fn add_renderer<T: Renderer + Clone>(&mut self, mut renderer: T) -> &Self {
        let id = Id::of::<T>();

        if !self.data.renderers_map.contains_key(&id) {
            renderer.set_scene_renderer(Some(self.clone()));
            let renderer_wrap = Shared::new(Box::new(renderer) as Box<Renderer>);
            self.data.renderers.push(renderer_wrap.clone());
            self.data.renderers_map.insert(id, renderer_wrap);
            self.sort_renderers();
        }
        self
    }
    pub fn has_renderer<T: Renderer + Clone>(&self) -> bool {
        self.data.renderers_map.contains_key(&Id::of::<T>())
    }
    pub fn remove_renderer<T: Renderer + Clone>(&mut self) -> &Self {
        let id = Id::of::<T>();

        if self.data.renderers_map.contains_key(&id) {
            {
                let ref mut renderers = self.data.renderers;
                match renderers.iter().position(|c| c.get_id() == id) {
                    Some(i) => {
                        renderers[i].set_scene_renderer(None);
                        renderers.remove(i);
                    },
                    None => (),
                }
            }
            self.data.renderers_map.remove(&id);
        }
        self
    }
    pub fn get_renderer<T: Renderer + Clone>(&self) -> Option<T> {
        let ref renderers_map = self.data.renderers_map;
        let id = Id::of::<T>();

        if renderers_map.contains_key(&id) {
            let renderer_ref = renderers_map.get(&id).unwrap();
            let renderer = renderer_ref.downcast_ref::<T>().unwrap();
            Some(renderer.clone())
        } else {
            None
        }
    }

    fn sort_renderers(&mut self) {
        self.data.renderers.sort_by(|a, b| {
            a.get_order().cmp(&b.get_order())
        });
    }
}
