use std::sync::{Arc, Mutex};

use gltf;
use crate::gltf_tree__::primitive__::Primitive;
use gloo_console::log;

pub struct Mesh {
    pub index: usize, // glTF index
    pub primitives: Vec<Arc<Mutex<Primitive>>>,
    // // TODO: weights
    // // pub weights: Vec<Rc<?>>
    // pub name: Option<String>,
    // // pub bounds: Aabb3<f32>,  // stripped collision
}

pub fn create_mesh
(
    g_mesh: Arc<gltf::Mesh>,
)
-> Mesh
{

    let primitives = g_mesh.primitives();


    log!("primitives.len() ", primitives.len());
    Mesh {
        index: 0,
        primitives: vec![],
    }
}