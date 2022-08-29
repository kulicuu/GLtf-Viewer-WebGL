use std::sync::{Arc, Mutex};

use gltf;

use crate::gltf_tree__::root__::Root;

use crate::gltf_tree__::math::*;

pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,

}

impl Default for Scene {
    fn default() -> Self {
        Self {
            name: None,
            nodes: vec![],
        }
    }
}

pub fn create_scene
(
    g_scene: &gltf::Scene,
    root: Arc<Mutex<Root>>,
)
-> Scene
{
    let mut scene = Scene {
        name: g_scene.name().map(|s| s.to_owned()),
        ..Default::default()
    };




    Scene::default()
}