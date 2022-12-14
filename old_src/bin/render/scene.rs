use gltf;

// use collision::{Aabb, Union};

use crate::controls::CameraParams;
use crate::render::root::Root;
use crate::render::math::*;

use std::sync::{Arc};

use gloo_console::log;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,
    // pub bounds: Aabb3,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            name: None,
            nodes: vec![],
            // bounds: Aabb3::zero()
        }
    }
}

impl Scene {
    pub fn from_gltf(g_scene: &gltf::Scene, root: &mut Root) -> Scene {
        let mut scene = Scene {
            name: g_scene.name().map(|s| s.to_owned()),
            ..Default::default()
        };
        scene.nodes = g_scene.nodes()
            .map(|g_node| g_node.index())
            .collect();

        // propagate transforms
        let root_transform = Matrix4::identity();
        for node_id in &scene.nodes {
            let node = root.unsafe_get_node_mut(*node_id);
            // node.update_transform(root, &root_transform);
            // node.update_bounds(root);
            // scene.bounds = scene.bounds.union(&node.bounds);
        }

        scene
    }

    // TODO: flatten draw call hierarchy (global Vec<Primitive>?)
    pub fn draw(
        &mut self, 
        gl: Arc<GL>,
        root: &mut Root, 
        cam_params: &CameraParams,
    ) 
    {
        // log!("Scene Draw");
        for node_id in &self.nodes {
            let node = root.unsafe_get_node_mut(*node_id);
            node.draw(
                gl.clone(),
                root, 
                cam_params,
            );
        }
    }
}
