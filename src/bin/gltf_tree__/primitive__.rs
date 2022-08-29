use std::sync::{Arc, Mutex};

use gltf;

use gloo_console::log;

use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::material__::{
    Material,
    create_material,
    create_shader_flags,
};
use crate::viewer__::ImportData;

use crate::gltf_tree__::math::*;
use crate::shader__::*;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


#[derive(Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tangent: Vector4,
    pub tex_coord_0: Vector2,
    pub tex_coord_1: Vector2,
    pub color_0: Vector4,
    pub joints_0: [u16; 4],
    pub weights_0: Vector4,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tangent: Vector4::zero(),
            tex_coord_0: Vector2::zero(),
            tex_coord_1: Vector2::zero(),
            color_0: Vector4::zero(),
            joints_0: [0; 4],
            weights_0: Vector4::zero(),
        }
    }
}


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
    import_data: Arc<ImportData>,
)
-> Primitive
{

    let buffers = &import_data.buffers;
    let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions = {
        let iter = reader
            .read_positions()
            .expect("positions failure.");
        iter.collect::<Vec<_>>()
    };

    let mut vertices: Vec<Vertex> = positions
        .into_iter()
        .map(|position| {
            Vertex {
                position: Vector3::from(position),
                ..Vertex::default()
            }
        }).collect();

    let mut shader_flags = ShaderFlags::empty();

    if let Some(normals) = reader.read_normals() {
        log!("Have normals in Primitive.");

    } else {
        log!("No normals.");
    }

    if let Some(tangents) = reader.read_tangents() {
        for (i, tangent) in tangents.enumerate() {
            vertices[i].tangent = Vector4::from(tangent);
        }
        shader_flags |= ShaderFlags::HAS_TANGENTS;
    }
    else {
        log!("No tangents.");
    }

    let mut tex_coord_set = 0;
    while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
        log!("Have tex coords");
        if tex_coord_set > 1 {
            tex_coord_set += 1;
            continue;
        }
        for (i, tex_coord) in tex_coords.into_f32().enumerate() {
            match tex_coord_set {
                0 => vertices[i].tex_coord_0 = Vector2::from(tex_coord),
                1 => vertices[i].tex_coord_1 = Vector2::from(tex_coord),
                _ => unreachable!()
            }
        }
        shader_flags |= ShaderFlags::HAS_UV;
        tex_coord_set += 1;
    }

    if let Some(colors) = reader.read_colors(0) {
        log!("Have colors");
        let colors = colors.into_rgba_f32();
        for (i, c) in colors.enumerate() {
            vertices[i].color_0 = c.into();
        }
        shader_flags |= ShaderFlags::HAS_COLORS;
    } else {
        log!("No colors.");
    }

    if reader.read_colors(1).is_some() {
        log!("Ignoring further color attributes, only supporting COLOR_0. (mesh");
    } else {
        log!("No extra color attributes.");
    }

    if let Some(joints) = reader.read_joints(0) {
        log!("Have joint");
        for (i, joint) in joints.into_u16().enumerate() {
            vertices[i].joints_0 = joint;
        }
    } else {
        log!("No joints.");
    }
    if reader.read_joints(1).is_some() {
        log!("Ignoring further joint attributes, only supporting JOINTS_0. (mesh: {}, primitive: {})");
    }

    if let Some(weights) = reader.read_weights(0) {
        for (i, weights) in weights.into_f32().enumerate() {
            vertices[i].weights_0 = weights.into();
        }
    }
    if reader.read_weights(1).is_some() {
        log!("Ignoring further weight attributes, only supporting WEIGHTS_0. (mesh: {}, primitive: {})");
    }

    let indices = reader
        .read_indices()
        .map(|read_indices| {
            log!("have index in reader of primitive");
            read_indices.into_u32().collect::<Vec<_>>()
    });
    
    let g_material = g_primitive.material();

    let mut material: Option<Arc<Mutex<Material>>> = None;
    if let Some(mat) = root.lock().unwrap().materials.iter().find(
        |m|
        m.lock().unwrap().index == g_material.index()
    )
    {
        log!("Found material in root.");
        material = Some(mat.clone());
    }

    if material.is_none() {
        log!("No material in root. To create:");
        material = Some(Arc::new(Mutex::new(
            create_material(
                g_material,
                root.clone(),
            )
        )));
        root.lock().unwrap().materials.push(material.clone().unwrap());
    };
    
    let material = material.unwrap();
    shader_flags |= create_shader_flags(material.clone());

    let mut new_shader = false;
    let shader = 
        if let Some(shader) = root.lock().unwrap().shaders.get(&shader_flags) {
            shader.clone()
        } else {
            new_shader = true;
            Arc::new(Mutex::new(create_pbr_shader().into()))
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