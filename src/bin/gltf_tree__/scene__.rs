use std::sync::{Arc, Mutex};

use gltf;

use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::node__::draw_node;

use crate::gltf_tree__::math::*;

pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,
}

use gloo_console::log;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

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


pub fn draw_scene
(
    gl: Arc<GL>,
    root: Arc<Mutex<Root>>,
    scene: Arc<Mutex<Scene>>,
    // camera_params: 
)
{
    for node_id in &scene.lock().unwrap().nodes {
        let node = root.lock().unwrap().nodes[*node_id].clone();
        draw_node(
            gl.clone(),
        );
    }

}