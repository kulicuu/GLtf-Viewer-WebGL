use std::sync::{Arc, Mutex};

use gltf;

use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::primitive__::{
    Primitive, 
    create_primitive, 
    draw_primitive
};

use crate::viewer__::ImportData;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

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
    gl: Arc<GL>,
    g_mesh: Arc<gltf::Mesh>,
    import_data: Arc<ImportData>,
    root: Arc<Mutex<Root>>,
)
-> Mesh
{

    let primitives: Vec<_>= g_mesh.primitives()
        .enumerate()
        .map(|(i, g_prim)| {
            log!("have a primitive in the mesh");
            Arc::new(
                Mutex::new(
                    create_primitive(
                        gl.clone(),
                        &g_prim,
                        i,
                        g_mesh.index(),
                        root.clone(),
                        import_data.clone(),
                    )
                )
            )
        })
        .collect();

    // log!("primitives.len() ", primitives.len());
    Mesh {
        index: 0,
        primitives,
    }
}

pub fn draw_mesh
(
    gl: Arc<GL>,
    mesh: Arc<Mutex<Mesh>>,
)
{
    for primitive in &mesh.lock().unwrap().primitives {
        draw_primitive(
            gl.clone(),
            primitive.clone(),
        )
    }

}