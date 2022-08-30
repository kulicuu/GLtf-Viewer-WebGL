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

use crate::gltf_tree__::root__::{create_root, Root};
use crate::gltf_tree__::scene__::{create_scene, Scene};
use crate::gltf_tree__::math::*;
use crate::controls::{OrbitControls};
use crate::gltf_tree__::camera__::Camera;

pub struct ImportData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub struct CameraOptions {
    pub index: i32,
    pub position: Option<Vector3>,
    pub target: Option<Vector3>,
    pub fovy: Deg<f32>,
    pub straight: bool,
}

pub fn prepare_gltf
(
    gl: Arc<GL>,
)
-> (
    Arc<Mutex<Root>>, 
    Arc<Mutex<Scene>>, 
    Arc<Mutex<OrbitControls>>,
)
{
    log!("Prepare Gltf.");
    let raw = include_bytes!("../../assets/Stork.glb");
    let (doc, buffers, images) = gltf::import_slice(raw).unwrap();

    let import_data = Arc::new(
            ImportData { doc, buffers, images }
    );

    let camera_options = CameraOptions {
        index: 0,
        position: Some(Vector3::new(0.3, 0.3, 0.3)),
        target: Some(Vector3::new(0.0, 0.0, 0.0)),
        fovy: Deg(35.0),
        straight: true,
    };

    let mut orbit_controls = OrbitControls::new(
        Point3::new(0.0, 0.0, 2.0),
        // inner_size,
    );
    orbit_controls.camera = Camera::default();
    orbit_controls.camera.fovy = camera_options.fovy;
    let orbit_controls = Arc::new(Mutex::new(
        orbit_controls,
    ));

    let root = create_root(
        gl.clone(),
        import_data.clone(),
    );
    
    // Only drawing one scene.  Default scene.
    let scene_index = 0;
    let scene = 
    Arc::new(Mutex::new(
        create_scene(
            &import_data.doc.scenes().nth(scene_index).unwrap(),
            root.clone(),
        )
    ));
    (
        root.clone(),
        scene.clone(),
        orbit_controls.clone(),
    )
    
}