


use gltf;
use std::sync::{Arc, Mutex};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};
use cgmath::{ Deg, Point3 };
use gloo_console::log;

use crate::gltf_tree__::root__::{create_root};
use crate::gltf_tree__::scene__::{create_scene};


pub struct ImportData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}



pub fn prepare_gltf
(
    gl: Arc<GL>,
)
{
    log!("Prepare Gltf.");
    let raw = include_bytes!("../../assets/Stork.glb");
    let (doc, buffers, images) = gltf::import_slice(raw).unwrap();

    let import_data = Arc::new(
        // Mutex::new(
            ImportData { doc, buffers, images }
        // )
    );

    let root = 
        create_root(
            gl.clone(),
            import_data.clone(),
        );
    


    let scene_index = 0;

    let scene = create_scene(
        &import_data.doc.scenes().nth(scene_index).unwrap(),
        root.clone(),
    );
    
}