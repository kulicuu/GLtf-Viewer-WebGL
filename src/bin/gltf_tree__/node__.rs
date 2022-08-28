
use std::sync::{Arc, Mutex};


use crate::viewer__::ImportData;
use crate::gltf_tree__::root__::Root;

use gltf;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use cgmath::Matrix4;

pub struct Node {
    // pub index: usize, // glTF index
    // pub children: Vec<usize>,
    // pub matrix: Matrix4<f32>,
    // pub mesh: Option<Arc<Mesh>>,
    // pub rotation: Quaternion,
    // pub scale: Vector3,
    // pub translation: Vector3,
    // // TODO: weights
    // // weights_id: usize,
    // pub camera: Option<Camera>,
    // pub name: Option<String>,

    // pub final_transform: Matrix4, // including parent transforms
    // // pub bounds: Aabb3,
}

pub fn create_node
(
    gl: Arc<GL>,
    g_node: &gltf::Node,
    root: Arc<Mutex<Root>>,
    import_data: Arc<ImportData>,
)
-> Node
{

    Node { }

}