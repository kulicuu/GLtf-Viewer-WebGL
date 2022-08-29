#![allow(unused)]

mod utils;
mod viewer__;
mod gltf_tree__;
mod shader__;
mod controls;

use crate::gltf_tree__::scene__::draw_scene;
use crate::controls::get_cam_params;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};
use serde_json::{Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::{Arc, Mutex};
use cgmath::prelude::*;
use cgmath::{Rad, Vector3, Vector4, Point3, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
// use std::{fs, io};
// use std::error::Error as stdError;
// use std::boxed::Box;
use gloo_console::log;
use std::f32::consts::PI;

use crate::utils::time_polyfill::Instant;

use gltf;



fn main()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();
    let gl : Arc<GL> = Arc::new(gl);
    




    let (root, scene, orbit_controls) = viewer__::prepare_gltf(gl.clone());

    let cam_params = Arc::new(Mutex::new(
        orbit_controls.lock().unwrap().camera_params()
    ));



    let start_time = Instant::now();
    let mut cursor: u128 = start_time.elapsed().as_millis();
    
    gl.clear_color(0.993, 0.9833, 0.952, 1.0);
    gl.enable(GL::DEPTH_TEST);

    let render_loop_closure = Rc::new(RefCell::new(None));
    let alias_rlc = render_loop_closure.clone();
    *alias_rlc.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        let now = start_time.elapsed().as_millis();  // total elapsed time from start
        let frame_delta = now - cursor;
        cursor = now;

        gl.clear_depth(1.0); // Clear everything
        gl.enable(GL::DEPTH_TEST); // Enable depth testing
        gl.depth_func(GL::LEQUAL);

        gl.clear(GL::COLOR_BUFFER_BIT);

        *cam_params.lock().unwrap() = orbit_controls.lock().unwrap().camera_params();

        draw_scene(
            gl.clone(),
            root.clone(),
            scene.clone(),
            cam_params.clone(),
        );

        request_animation_frame(render_loop_closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(alias_rlc.borrow().as_ref().unwrap());    
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}



