
use std::sync::{Arc, Mutex};


use crate::viewer__::ImportData;
use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::mesh__::{Mesh, create_mesh, draw_mesh};

use crate::gltf_tree__::math::*;
use crate::controls::{CameraParams};
use gltf;

use gloo_console::log;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};




pub struct Node {
    pub index: usize, // glTF index
    pub children: Vec<usize>,
    pub matrix: Matrix4,
    pub mesh: Option<Arc<Mutex<Mesh>>>,
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


    let matrix = &g_node.transform().matrix();
    let matrix: &Matrix4 = matrix.into();
    let matrix = *matrix;

    let (trans, gn_rot, scale) = g_node.transform().decomposed();
    let r = gn_rot;
    let rotation = Quaternion::new(r[3], r[0], r[1], r[2]);

    let mut mesh = None;
    if let Some(g_mesh) = g_node.mesh() {
        let g_mesh = Arc::new(g_mesh);
        if let Some(existing_mesh) = root.lock().unwrap().meshes.iter().find(|mesh| mesh.lock().unwrap().index == g_mesh.index()) {
            log!("existing mesh.");
            mesh = Some(Arc::new(
                Mutex::new(
                    create_mesh(
                        gl.clone(),
                        g_mesh.clone(),
                        import_data.clone(),
                        root.clone(),
                    )
                )
            ));
        }
        if mesh.is_none() {
            mesh = Some(Arc::new(
                Mutex::new(
                    create_mesh(
                        gl.clone(),
                        g_mesh.clone(),
                        import_data.clone(),
                        root.clone(),
                    )
                )
            ));
        }

    }

    let children: Vec<_> = g_node.children()
        .map(|g_node| g_node.index())
        .collect();

    Node {
        index: g_node.index(),
        children,
        matrix,
        mesh: mesh.clone(),
     }

}


pub fn draw_node
(
    gl: Arc<GL>,
    root: Arc<Mutex<Root>>,
    node: Arc<Mutex<Node>>,
    cam_params: Arc<Mutex<CameraParams>>,

)
{
    if let Some(ref mesh) = node.lock().unwrap().mesh {
        draw_mesh(
            gl.clone(),
            (*mesh).clone(),
        );        
    }
    for node_id in &node.lock().unwrap().children {
        let node = root.lock().unwrap().nodes[*node_id].clone();
        draw_node(
            gl.clone(),
            root.clone(),
            node,
            cam_params.clone(),
        );
    }
}