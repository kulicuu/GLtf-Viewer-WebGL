use std::sync::{Arc, Mutex};

use gltf;

use gloo_console::log;

use crate::gltf_tree__::root__::Root;
use crate::viewer__::ImportData;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


pub struct Primitive {
    // gl: Arc<GL>,
    // // vao: u32,
    // // vbo: u32,
    // // num_vertices: u32,

    // // ebo: Option<u32>,
    // num_indices: u32,

    // material: Rc<Material>,

    // // pbr_shader: Rc<PbrShader>,
    // pbr_shader: Arc<PbrShader>,

    // // TODO!: mode, targets
}


pub fn create_primitive
(
    g_primitive: &gltf::Primitive,
    i: usize,
    idx: usize,
    root: Arc<Mutex<Root>>,
    import_data: Arc<Mutex<ImportData>>,
)
-> Primitive
{

    let buffers = &import_data.lock().unwrap().buffers;
    let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions = {
        let iter = reader
            .read_positions()
            .expect("positions failure.");
        iter.collect::<Vec<_>>()
    };











    Primitive {}
}


fn prepare_draw
()
{

}

fn per_frame_configure
()
{

}

fn draw
()
{

}