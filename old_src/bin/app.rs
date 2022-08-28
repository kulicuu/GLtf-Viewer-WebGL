#![allow(unused)]

mod utils;
mod render;
mod controls;
mod shader;
mod import_data;
mod viewer;

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


// Organizing state mgmt and draw operations here.
// Maybe transport stuff from the object-oriented legacy code 
// to something more procedural.




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

    

    
    let mut viewer_to_draw = viewer::GltfViewer::new(gl.clone());
    
    
    let start_time = Instant::now();
    let mut cursor: u128 = start_time.elapsed().as_millis();

    
    
    gl.clear_color(0.993, 0.9833, 0.952, 1.0);
    gl.enable(GL::DEPTH_TEST);


    let state: [u32; 1] = [0]; // can't do much of anything with this but hold the place.

    

    // We'll import some functions here.





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

        viewer_to_draw.draw(gl.clone());

        request_animation_frame(render_loop_closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(alias_rlc.borrow().as_ref().unwrap());    
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}



fn import_1() {
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
}
