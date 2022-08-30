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
    gl: Arc<GL>,
    // // vao: u32,
    // // vbo: u32,
    num_vertices: u32,
    // // ebo: Option<u32>,
    num_indices: u32,
    material: Arc<Mutex<Material>>,
    pbr_shader: Arc<Mutex<PbrShader>>,
}


pub fn create_primitive
(
    gl: Arc<GL>,
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

    if new_shader {
        root.lock().unwrap().shaders.insert(
            shader_flags,
            shader.clone(),
        );
    }

    let num_indices = indices.as_ref().map(|i| i.len()).unwrap_or(0);

    prepare_draw(
        gl.clone(),
        &vertices,
        indices,
    );

    Primitive {
        gl: gl.clone(),
        num_vertices: 10,
        num_indices: 10,
        material: material.clone(),
        pbr_shader: shader.clone(),
    }
}


fn prepare_draw
(
    gl: Arc<GL>,
    vertices: &[Vertex],
    indices: Option<Vec<u32>>,
)
{
    let mut vertices_positions = vec![];
    let mut normals = vec![];
    for vertex in vertices {
        vertices_positions.extend([vertex.position[0], vertex.position[1], vertex.position[2]].iter().copied());
        normals.extend([vertex.normal[0], vertex.normal[1], vertex.normal[2]].iter().copied());
    }

    let vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_verts = js_sys::Float32Array::from(vertices_positions.as_slice());

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_verts, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(0 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0 as u32);

    let normals_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_normals = js_sys::Float32Array::from(normals.as_slice());

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normals_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_normals, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(1 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(1 as u32);

    let index_buffer = Arc::new(gl.create_buffer().unwrap());
    let indices = indices.unwrap();
    let js_indices = js_sys::Uint32Array::from(indices.as_slice());

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &js_indices, GL::STATIC_DRAW);
}

fn per_frame_configure
(
    gl: Arc<GL>,
    primitive: Arc<Mutex<Primitive>>,
    
    model_matrix: &Matrix4,
    mvp_matrix: &Matrix4,
    camera_position: &Vector3,
)
{
    log!("Per frame configure.");
    let material = primitive.lock().unwrap().material.clone();


}

pub fn draw_primitive
(
    gl: Arc<GL>,
    primitive: Arc<Mutex<Primitive>>,
    model_matrix: &Matrix4,
    mvp_matrix: &Matrix4,
    camera_position: &Vector3,
)
{

    per_frame_configure(
        gl.clone(),
        primitive.clone(),
        model_matrix,
        mvp_matrix,
        camera_position,
    );

    // gl.draw_elements...

}