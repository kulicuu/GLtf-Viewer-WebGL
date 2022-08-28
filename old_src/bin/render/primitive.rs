use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use std::rc::Rc;

// use gl;
use gltf;
use gltf::json::mesh::Mode;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use gloo_console::log;

use std::sync::Arc;

extern crate mat4;

// use camera::Camera;
use crate::render::math::*;
use crate::render::material::Material;
use crate::render::root::Root;
use crate::shader::*;
use crate::shader::ShaderFlags;
// use crate::import_data::ImportData;
use crate::render::texture::ImportData;

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

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub struct Primitive {
    gl: Arc<GL>,
    // vao: u32,
    // vbo: u32,
    // num_vertices: u32,

    // ebo: Option<u32>,
    num_indices: u32,

    material: Rc<Material>,

    // pbr_shader: Rc<PbrShader>,
    pbr_shader: Arc<PbrShader>,

    // TODO!: mode, targets
}


// Might as well use Arcs for option to multithread.    
impl Primitive {
    pub fn new(
        gl: Arc<GL>,
        // bounds: Aabb3,
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
        material: Rc<Material>,
        shader: Arc<PbrShader>,
    ) -> Primitive {
        let num_indices = indices.as_ref().map(|i| i.len()).unwrap_or(0);
        let mut prim = Primitive {
            gl: gl.clone(),
            // bounds,
            // num_vertices: vertices.len() as u32,
            num_indices: num_indices as u32,
            // vao: 0, vbo: 0, ebo: None,
            material,
            pbr_shader: shader,
        };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        // prim.setup_primitive(gl.clone(), vertices, indices);
        prim
    }

    pub fn from_gltf(
        gl: Arc<GL>,
        g_primitive: &gltf::Primitive,
        primitive_index: usize,
        mesh_index: usize,
        root: &mut Root,
        imp: &ImportData,
    )
    -> Primitive
    {
        let buffers = &imp.buffers;
        let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions = {
            let iter = reader
                .read_positions()
                .expect(&format!(
                    "primitives must have the POSITION attribute (mesh: {}, primitive: {})",
                    mesh_index, primitive_index));
            iter.collect::<Vec<_>>()
        };

        // let bounds = g_primitive.bounding_box();
        // let bounds = Aabb3 {
        //     min: bounds.min.into(),
        //     max: bounds.max.into()
        // };

        let mut vertices: Vec<Vertex> = positions
            .into_iter()
            .map(|position| {
                Vertex {
                    position: Vector3::from(position),
                    ..Vertex::default()
                }
            }).collect();

        let mut shader_flags = ShaderFlags::empty();

        // normals
        if let Some(normals) = reader.read_normals() {
            for (i, normal) in normals.enumerate() {
                vertices[i].normal = Vector3::from(normal);
            }
            shader_flags |= ShaderFlags::HAS_NORMALS;
        }
        else {
            // debug!("Found no NORMALs for primitive {} of mesh {} \
            //        (flat normal calculation not implemented yet)", primitive_index, mesh_index);
        }

        // tangents
        if let Some(tangents) = reader.read_tangents() {
            log!("have tangents.");
            for (i, tangent) in tangents.enumerate() {
                vertices[i].tangent = Vector4::from(tangent);
            }
            shader_flags |= ShaderFlags::HAS_TANGENTS;
        }
        else {
            log!("no tangents.");
            // debug!("Found no TANGENTS for primitive {} of mesh {} \
            //        (tangent calculation not implemented yet)", primitive_index, mesh_index);
        }

        // texture coordinates
        let mut tex_coord_set = 0;
        while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
            if tex_coord_set > 1 {
                log!("have tex coords");
                // warn!("Ignoring texture coordinate set {}, \
                //         only supporting 2 sets at the moment. (mesh: {}, primitive: {})",
                //         tex_coord_set, mesh_index, primitive_index);
                tex_coord_set += 1;
                continue;
            }
            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                log!("tex coord");
                match tex_coord_set {
                    0 => vertices[i].tex_coord_0 = Vector2::from(tex_coord),
                    1 => vertices[i].tex_coord_1 = Vector2::from(tex_coord),
                    _ => unreachable!()
                }
            }
            shader_flags |= ShaderFlags::HAS_UV;
            tex_coord_set += 1;
        }

        // colors
        if let Some(colors) = reader.read_colors(0) {
            let colors = colors.into_rgba_f32();
            for (i, c) in colors.enumerate() {
                vertices[i].color_0 = c.into();
            }
            shader_flags |= ShaderFlags::HAS_COLORS;
        }
        if reader.read_colors(1).is_some() {
            // warn!("Ignoring further color attributes, only supporting COLOR_0. (mesh: {}, primitive: {})",
                // mesh_index, primitive_index);
        }

        if let Some(joints) = reader.read_joints(0) {
            for (i, joint) in joints.into_u16().enumerate() {
                vertices[i].joints_0 = joint;
            }
        }
        if reader.read_joints(1).is_some() {
            // warn!("Ignoring further joint attributes, only supporting JOINTS_0. (mesh: {}, primitive: {})",
                // mesh_index, primitive_index);
        }

        if let Some(weights) = reader.read_weights(0) {
            log!("have weights.");
            for (i, weights) in weights.into_f32().enumerate() {
                vertices[i].weights_0 = weights.into();
            }
        }
        if reader.read_weights(1).is_some() {
            // warn!("Ignoring further weight attributes, only supporting WEIGHTS_0. (mesh: {}, primitive: {})",
            //     mesh_index, primitive_index);
        }

        let indices = reader
            .read_indices()
            .map(|read_indices| {
                log!("have index.");
                read_indices.into_u32().collect::<Vec<_>>()
            });

        // assert_eq!(g_primitive.mode(), Mode::Triangles, "not yet implemented: primitive mode must be Triangles.");

        let g_material = g_primitive.material();

        let mut material = None;
        if let Some(mat) = root.materials.iter().find(|m| (***m).index == g_material.index()) {
            // log!("have Some (mat)");
            material = Rc::clone(mat).into()
        }

        if material.is_none() { // no else due to borrow checker madness
            let mat = Rc::new(Material::from_gltf(
                gl.clone(),
                &g_material, 
                root, 
                imp
            ));
            root.materials.push(Rc::clone(&mat));
            material = Some(mat);
        };
        let material = material.unwrap();
        shader_flags |= material.shader_flags();

        let mut new_shader = false; // borrow checker workaround
        let shader =
            if let Some(shader) = root.shaders.get(&shader_flags) {
                shader.clone()
            }
            else {
                log!("Else branch creating new.");
                new_shader = true;
                // PbrShader::new(shader_flags).into()
                Arc::new(PbrShader::new(
                    gl.clone(),
                    shader_flags,
                ))
            };
        if new_shader {
            root.shaders.insert(shader_flags, shader.clone());
        }

        Primitive::new(
            gl.clone(), 
            // bounds, 
            &vertices, 
            indices, 
            material, 
            shader
        )
    }

    /// render the mesh
    pub fn draw(
        &self, 
        gl: Arc<GL>,
        model_matrix: &Matrix4, 
        mvp_matrix: &Matrix4, 
        camera_position: &Vector3
    ) {
        // log!("Primitive Draw");
        // TODO!: determine if shader+material already active to reduce work...

        // if self.material.double_sided {
        //     gl.disable(GL::CULL_FACE);
        // } else {
        //     gl.enable(GL::CULL_FACE);
        // }

        self.configure_shader(
            gl.clone(),
            model_matrix, 
            mvp_matrix, 
            camera_position
        );

        // // draw mesh
        // gl::BindVertexArray(self.vao);
        // if self.ebo.is_some() {
        //     gl::DrawElements(gl::TRIANGLES, self.num_indices as i32, gl::UNSIGNED_INT, ptr::null());
        // }
        // else {
        //     gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices as i32)
        // }

        // gl::BindVertexArray(0);
        // gl::ActiveTexture(gl::TEXTURE0);
    }

    fn configure_shader(
        &self, 
        gl: Arc<GL>,
        model_matrix: &Matrix4,
        mvp_matrix: &Matrix4, 
        camera_position: &Vector3,
    )
    {
        log!("Configure Shader"); // This procedure is running every frame.

        // let pbr_shader = &Rc::get_mut(&mut self.pbr_shader).unwrap();
        let pbr_shader = self.pbr_shader.clone();

        let mat = &self.material;
        // let shader = pbr_shader.shader;


        let mat = &self.material;
        let shader = &self.pbr_shader.shader;
        let uniform_locations = &self.pbr_shader.uniform_locations;
        
        // self.pbr_shader.shader.use_program();
        gl.link_program(&shader);

        // camera params

        let mut arr: [f32; 30] = [0.0; 30];
        let mut kdx: usize = 0;
        for idx in 0..4 {
            for jdx in 0..4 {
                arr[kdx] = model_matrix[idx][jdx];
                // log!("view_mat: ", view_mat[idx][jdx]);
                kdx = kdx + 1;
            }
        }
        
        let arr_js = js_sys::Float32Array::from(arr.as_slice());


        // let mut projectionMatrix = mat4::new_zero();

        // gl.uniform_matrix4fv_with_f32_array(
        //     Some(&uniform_locations.u_ModelMatrix),
        //     false,
        //     &projectionMatrix,
        // );
        
        // let x = gl.get_uniform_location(&shader, "u_MVPMatrix").unwrap();

        // gl.uniform_matrix4fv_with_f32_array(
        //     Some(&(&uniform_locations.u_MVPMatrix)),
        //     false,
        //     &projectionMatrix,
        // );


    

        // shader.set_mat4(uniforms.u_ModelMatrix, model_matrix);
        // shader.set_mat4(uniforms.u_MVPMatrix, mvp_matrix);
        // shader.set_vector3(uniforms.u_Camera, camera_position);

        
        
        
        
        
        
        
        
        
        
        // // NOTE: for sampler numbers, see also PbrShader constructor
        // shader.set_vector4(uniforms.u_BaseColorFactor, &mat.base_color_factor);
        // if let Some(ref base_color_texture) = mat.base_color_texture {
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     gl::BindTexture(gl::TEXTURE_2D, base_color_texture.id);
        // }
        // if let Some(ref normal_texture) = mat.normal_texture {
        //     gl::ActiveTexture(gl::TEXTURE1);
        //     gl::BindTexture(gl::TEXTURE_2D, normal_texture.id);
        // }
        // if let Some(ref emissive_texture) = mat.emissive_texture {
        //     gl::ActiveTexture(gl::TEXTURE2);
        //     gl::BindTexture(gl::TEXTURE_2D, emissive_texture.id);

        //     shader.set_vector3(uniforms.u_EmissiveFactor, &mat.emissive_factor);
        // }

        // if let Some(ref mr_texture) = mat.metallic_roughness_texture {
        //     gl::ActiveTexture(gl::TEXTURE3);
        //     gl::BindTexture(gl::TEXTURE_2D, mr_texture.id);
        // }
        // shader.set_vec2(uniforms.u_MetallicRoughnessValues,
        //     mat.metallic_factor, mat.roughness_factor);

        // if let Some(ref occlusion_texture) = mat.occlusion_texture {
        //     gl::ActiveTexture(gl::TEXTURE4);
        //     gl::BindTexture(gl::TEXTURE_2D, occlusion_texture.id);

        //     shader.set_float(uniforms.u_OcclusionStrength, mat.occlusion_strength);
        // }
    }


    fn setup_primitive(
        &mut self, 
        gl: Arc<GL>,
        vertices: &[Vertex], 
        indices: Option<Vec<u32>>)
         {
            log!("setup_primitive.");
            // let vertices_position = gl.get_attrib_location(
            //     &self.pbr_shader.shader,
            //     "a_Position",
            // ) as i32;

            
            // let vertex_buffer = gl.create_buffer().unwrap();
            

            // vertices.iter().map(|item| {
            //     log!("item");
            // });


            // let js_verts = js_sys::Float32Array::from(vertices.as_slice());

        // create buffers/arrays
        // gl::GenVertexArrays(1, &mut self.vao);
        // gl::GenBuffers(1, &mut self.vbo);
        // if indices.is_some() {
        //     let mut ebo = 0;
        //     gl::GenBuffers(1, &mut ebo);
        //     self.ebo = Some(ebo);
        // }

        // gl::BindVertexArray(self.vao);
        // // load data into vertex buffers
        // gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        // let size = (vertices.len() * size_of::<Vertex>()) as isize;
        // let data = &vertices[0] as *const Vertex as *const c_void;
        // gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // if let Some(ebo) = self.ebo {
        //     let indices = indices.unwrap();
        //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        //     let size = (indices.len() * size_of::<u32>()) as isize;
        //     let data = &indices[0] as *const u32 as *const c_void;
        //     gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        // }

        // // set the vertex attribute pointers
        // let size = size_of::<Vertex>() as i32;
        // // POSITION
        // gl::EnableVertexAttribArray(0);
        // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);
        // // NORMAL
        // gl::EnableVertexAttribArray(1);
        // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);
        // // TANGENT
        // gl::EnableVertexAttribArray(2);
        // gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tangent) as *const c_void);
        // // TEXCOORD_0
        // gl::EnableVertexAttribArray(3);
        // gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coord_0) as *const c_void);
        // // TEXCOORD_1
        // gl::EnableVertexAttribArray(4);
        // gl::VertexAttribPointer(4, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coord_1) as *const c_void);
        // // COLOR_0
        // gl::EnableVertexAttribArray(5);
        // gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, color_0) as *const c_void);
        // // JOINTS_0
        // gl::EnableVertexAttribArray(6);
        // // TODO: normalization?
        // gl::VertexAttribPointer(6, 4, gl::UNSIGNED_SHORT, gl::FALSE, size, offset_of!(Vertex, joints_0) as *const c_void);
        // // WEIGHTS_0
        // gl::EnableVertexAttribArray(7);
        // gl::VertexAttribPointer(7, 4, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, weights_0) as *const c_void);

        // gl::BindVertexArray(0);
    }
}
