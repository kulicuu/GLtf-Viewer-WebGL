

use gltf;


use std::sync::{Arc};


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use cgmath::{ Deg, Point3 };

use crate::render::math::*;

use crate::render::scene::Scene;
use crate::render::root::Root;
use crate::render::camera::Camera;


use crate::controls::OrbitControls;

use crate::render::texture::ImportData;
// use crate::import_data::ImportData;



use gloo_console::log;





pub fn load_1
()
{
    let raw = include_bytes!("../../assets/Stork.glb");
    let (document, buffers, images) = gltf::import_slice(raw).unwrap();

    let raw = include_bytes!("../../assets/Stork.glb");
    let parsed = gltf::Glb::from_slice(raw).unwrap();
    let header = parsed.header;
    log!("header: ", header.length);

    let s = match std::str::from_utf8(&*(parsed.json)) {
        Ok(v) => v,
        Err(e) => "Error parsing utf8",
    };
    let bin = parsed.bin.unwrap();
    log!("json:", s);
    log!("bin.len(): ", bin.len());


}


pub struct CameraOptions {
    pub index: i32,
    pub position: Option<Vector3>,
    pub target: Option<Vector3>,
    pub fovy: Deg<f32>,
    pub straight: bool,
}



pub struct GltfViewer {
    // size: PhysicalSize,
    // dpi_factor: f64,

    orbit_controls: OrbitControls,
    // first_mouse: bool,
    // last_x: f32,
    // last_y: f32,
    // events_loop: Option<glutin::EventsLoop>,
    // gl_window: Option<glutin::GlWindow>,

    // TODO!: get rid of scene?
    root: Root,
    scene: Scene,

    delta_time: f64, // seconds
    // last_frame: Instant,

    // render_timer: FrameTimer,
}

impl GltfViewer {
    pub fn new(
        gl: Arc<GL>,
        // camera_options: CameraOptions,
    ) 
    -> GltfViewer 
    {

        let camera_options = CameraOptions {
            index: 0,
            position: Some(Vector3::new(0.3, 0.3, 0.3)),
            target: Some(Vector3::new(0.0, 0.0, 0.0)),
            fovy: Deg(35.0),
            straight: true,
        };


        let raw = include_bytes!("../../assets/Stork.glb");
        let (doc, buffers, images) = gltf::import_slice(raw).unwrap();
    
        let raw = include_bytes!("../../assets/Stork.glb");
        let parsed = gltf::Glb::from_slice(raw).unwrap();
        let header = parsed.header;
        log!("header: ", header.length);
    
        let s = match std::str::from_utf8(&*(parsed.json)) {
            Ok(v) => v,
            Err(e) => "Error parsing utf8",
        };
        let bin = parsed.bin.unwrap();
        log!("json:", s);
        log!("bin.len(): ", bin.len());

        let imp = ImportData { doc, buffers, images };

        let mut root = Root::from_gltf(gl.clone(), &imp);
        let scene_index = 0;
        let scene = Scene::from_gltf(
            &imp.doc.scenes().nth(scene_index).unwrap(), 
            &mut root
        );



        let mut orbit_controls = OrbitControls::new(
            Point3::new(0.0, 0.0, 2.0),
            // inner_size,
        );
        orbit_controls.camera = Camera::default();
        orbit_controls.camera.fovy = camera_options.fovy;
        // orbit_controls.camera.update_aspect_ratio(inner_size.width as f32 / inner_size.height as f32); // updates projection matrix

        // let (root, scene) = Self::load(source, scene_index);
        let mut viewer = GltfViewer {
            // size: inner_size,
            // dpi_factor,

            orbit_controls,
            // first_mouse, last_x, last_y,

            // events_loop,
            // gl_window,

            root,
            scene,

            delta_time: 0.0, // seconds
            // last_frame: Instant::now(),

            // render_timer: FrameTimer::new("rendering", 300),
        };
        viewer
    }

    pub fn draw(
        &mut self,
        gl: Arc<GL>,
    ) {
        // log!("Viewer Draw. ");
        let cam_params = self.orbit_controls.camera_params();
        self.scene.draw(
            gl.clone(),
            &mut self.root, 
            &cam_params,
        );

    }




}