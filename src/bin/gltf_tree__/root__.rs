
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};
use crate::viewer__::ImportData;
use crate::gltf_tree__::node__::{Node, create_node};
use crate::gltf_tree__::mesh__::Mesh;
use crate::gltf_tree__::material__::Material;

use crate::shader__::{ShaderFlags, PbrShader};
use gloo_console::log;

pub struct Root {
    pub nodes: Vec<Arc<Mutex<Node>>>,
    pub meshes: Vec<Arc<Mutex<Mesh>>>, 
    pub materials: Vec<Arc<Mutex<Material>>>,
    // pub textures: Vec<Arc<Mutex<Texture>>>,
    
    pub shaders: HashMap<ShaderFlags, Arc<Mutex<PbrShader>>>,
    // pub camera_nodes: Vec<usize>, //indices of cameras

    // TODO!: joint_nodes, mesh_nodes?
}



pub fn create_root
(
    gl: Arc<GL>,
    import_data: Arc<ImportData>,
)
{
    let root = Arc::new(
        Mutex::new(
            Root { 
                nodes: vec![],
                meshes: vec![],
                materials: vec![],
                // textures: vec![],
                shaders: HashMap::new(),
                // camera_nodes: vec![],
            }
        )
    );


    // log!("create root.", import_data.doc.nodes().len());
    // let nodes: Vec<Arc<Mutex<Node>>> = import_data.doc.nodes()
    root.lock().unwrap().nodes = import_data.doc.nodes()
        .map(|g_node| {
            log!("here node.");
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
        }).collect();
        // let camera_nodes = root.lock().unwrap().nodes.iter()
        //     .filter(|node| node.lock().unwrap().camera.is_some())
        //     .map(|node| node.index)
        //     .collect();




}