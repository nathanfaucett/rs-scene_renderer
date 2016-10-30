use alloc::boxed::Box;

use scene_graph::{Id, Scene};
use hash_map::HashMap;
use vector::Vector;
use iterable::Iterable;
use iterable_mut::IterableMut;
use map::Map;
use stack::Stack;
use insert::Insert;
use remove::Remove;
use shared::Shared;

use renderer::Renderer;
use plugin::Plugin;


struct SceneRendererData {
    scene: Scene,

    initted: bool,

    renderers_map: HashMap<Id, Shared<Box<Renderer>>>,
    renderers: Vector<Shared<Box<Renderer>>>,

    plugins_map: HashMap<Id, Shared<Box<Plugin>>>,
    plugins: Vector<Shared<Box<Plugin>>>,
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

                initted: false,

                renderers_map: HashMap::new(),
                renderers: Vector::new(),

                plugins_map: HashMap::new(),
                plugins: Vector::new(),
            })
        }
    }

    pub fn get_scene(&self) -> Scene {
        self.data.scene.clone()
    }

    pub fn init(&mut self) -> &mut Self {
        if !self.data.initted {
            self.data.initted = true;

            self.sort_plugins();
            self.sort_renderers();

            for plugin in self.data.plugins.iter_mut() {
                plugin.init();
            }
            for renderer in self.data.renderers.iter_mut() {
                renderer.init();
            }
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        for renderer in self.data.renderers.iter_mut() {
            renderer.clear();
        }
        for plugin in self.data.plugins.iter_mut() {
            plugin.clear();
        }
        {
            let ref mut data = self.data;

            data.initted = false;

            data.renderers_map.clear();
            data.renderers.clear();

            data.plugins_map.clear();
            data.plugins.clear();
        }
        self
    }

    pub fn render(&mut self) -> &mut Self {
        for plugin in self.data.plugins.iter_mut() {
            plugin.before_render();
        }
        for renderer in self.data.renderers.iter_mut() {
            renderer.before_render();
            renderer.render();
            renderer.after_render();
        }
        for plugin in self.data.plugins.iter_mut() {
            plugin.after_render();
        }
        self
    }

    pub fn add_renderer<T: Renderer + Clone>(&mut self, mut renderer: T) -> &Self {
        let id = Id::of::<T>();

        if !self.data.renderers_map.contains_key(&id) {
            renderer.set_scene_renderer(Some(self.clone()));
            let renderer_wrap = Shared::new(Box::new(renderer.clone()) as Box<Renderer>);
            self.data.renderers.push(renderer_wrap.clone());
            self.data.renderers_map.insert(id, renderer_wrap);
            self.sort_renderers();

            if self.data.initted {
                renderer.init();
            }
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
                        {
                            let ref mut renderer = renderers[i];
                            renderer.clear();
                            renderer.set_scene_renderer(None);
                        }
                        renderers.remove(&i);
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

    pub fn add_plugin<T: Plugin + Clone>(&mut self, mut plugin: T) -> &mut Self {
        let shared_plugin = Shared::new(Box::new(plugin.clone()) as Box<Plugin>);

        self.data.plugins_map.insert(plugin.get_id(), shared_plugin.clone());
        self.data.plugins.push(shared_plugin);

        plugin.set_scene_renderer(Some(self.clone()));

        if self.data.initted {
            self.sort_plugins();
        }

        self
    }
    pub fn has_plugin<T: Plugin>(&self) -> bool {
        self.data.plugins_map.contains_key(&Id::of::<T>())
    }
    pub fn remove_plugin<T: Plugin + Clone>(&mut self, mut plugin: T) -> &mut Self {
        if plugin.get_scene_renderer().is_none() {
            return self;
        }
        let id = plugin.get_id();

        self.data.plugins_map.remove(&id);
        {
            let ref mut plugins = self.data.plugins;
            match plugins.iter().position(|p| p.get_id() == id) {
                Some(i) => {
                    plugins.remove(&i);
                },
                None => {},
            }
        }
        plugin.set_scene_renderer(None);
        plugin.clear();

        self
    }

    pub fn get_plugin<T: Plugin + Clone>(&self) -> Option<T> {
        let ref plugins_map = self.data.plugins_map;
        let id = Id::of::<T>();

        if plugins_map.contains_key(&id) {
            let ref_plugin = plugins_map.get(&id).unwrap();
            let plugin = ref_plugin.downcast_ref::<T>().unwrap();
            Some(plugin.clone())
        } else {
            None
        }
    }
    pub fn for_each_plugin<F>(&mut self, func: F) where F: Fn(&mut Box<Plugin>) {
        for plugin in self.data.plugins.iter_mut() {
            func(plugin);
        }
    }
    fn sort_plugins(&mut self) {
        self.data.plugins.sort_by(|a, b| {
            a.get_order().cmp(&b.get_order())
        });
    }
}
