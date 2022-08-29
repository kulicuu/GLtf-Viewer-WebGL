use std::sync::{Arc, Mutex};


use gltf;
use crate::gltf_tree__::primitive__::Primitive;
use crate::viewer__::ImportData;
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
    import_data: Arc<ImportData>,
)
-> Mesh
{

    // let primitives = g_mesh.primitives();

    let primitives: Vec<_>= g_mesh.primitives()
    // let primitives: Vec<()> = g_mesh.primitives()
        .enumerate()
        .map(|(i, g_prim)| {
            log!("have a primitive in the mesh");
            // Primitive::from_gltf(
            //     gl.clone(),
            //     &g_prim, 
            //     i, 
            //     g_mesh.index(), 
            //     root, 
            //     imp
            // )
        })
        .collect();

    // log!("primitives.len() ", primitives.len());
    Mesh {
        index: 0,
        primitives: vec![],
    }
}