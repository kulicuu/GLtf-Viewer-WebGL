use gltf;
use std::sync::{Arc, Mutex};
use crate::viewer__::ImportData;
use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::mesh__::{Mesh, create_mesh, draw_mesh};
use crate::gltf_tree__::math::*;
use crate::gltf_tree__::camera__::{Camera};
use crate::controls::{CameraParams};
use gloo_console::log;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

pub struct Node {
    pub index: usize, // glTF index
    // pub children: Vec<usize>,
    pub children: Vec<Arc<Mutex<Node>>>,
    pub matrix: Matrix4,
    pub mesh: Option<Arc<Mutex<Mesh>>>,
    pub rotation: Quaternion,
    // pub scale: Vector3,
    pub translation: Vector3,
    // // TODO: weights
    // // weights_id: usize,
    pub camera: Option<Camera>,
    pub name: Option<String>,
    pub final_transform: Matrix4, // including parent transforms
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

    // shouldn't these nodes also need to be constructed? Here we only get indices.  
    // 
    // let children: Vec<_> = g_node.children() 
    //     .map(|g_node| g_node.index())
    //     .collect();


    // shouldn't these nodes also need to be constructed? Here we only get indices.
    // No.  It looks like all the child nodes are included in the main nodes array?
    // And these are indexed by the children array of indices.  


    let children: Vec<Arc<Mutex<Node>>> = g_node.children() 
        .map(|g_node| {
            Arc::new(
                Mutex::new(
                    create_node(
                        gl.clone(),
                        &g_node,
                        root.clone(),
                        import_data.clone(),  
                    )
                )
            )
        })
        .collect();




    Node {
        index: g_node.index(),
        children,
        matrix,
        mesh: mesh.clone(),
        rotation,
        translation: trans.into(),
        camera: g_node.camera().as_ref().map(Camera::from_gltf),
        final_transform: Matrix4::identity(),
        name: g_node.name().map(|s| s.into()),
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
    let final_transform = node.lock().unwrap().final_transform;
    if let Some(ref mesh) = node.lock().unwrap().mesh {
        let mut r = cam_params.lock().unwrap();
        let mvp_matrix = r.projection_matrix * r.view_matrix;
        draw_mesh(
            gl.clone(),
            (*mesh).clone(),
            &final_transform,
            &mvp_matrix,
            &r.position,
        );        
    }
    for node_w in &node.lock().unwrap().children {
        draw_node(
            gl.clone(),
            root.clone(),
            node_w.clone(),
            cam_params.clone(),
        );
    }
}