use gltf;

use std::sync::{Arc, Mutex};

use crate::shader__::*;

use crate::gltf_tree__::root__::Root;
use crate::gltf_tree__::texture__::Texture;

use gloo_console::log;


pub struct Material {
    // gl: Arc<GL>,
    pub index: Option<usize>, 
    // pub name: Option<String>,

    // // pbr_metallic_roughness properties
    // // pub base_color_factor: Vector4,
    pub base_color_texture: Option<Arc<Mutex<Texture>>>,
    // // pub metallic_factor: f32,
    // // pub roughness_factor: f32,
    // pub metallic_roughness_texture: Option<Rc<Texture>>,

    // pub normal_texture: Option<Rc<Texture>>,
    // pub normal_scale: Option<f32>,

    // pub occlusion_texture: Option<Rc<Texture>>,
    // pub occlusion_strength: f32,
    // pub emissive_factor: Vector3,
    // pub emissive_texture: Option<Rc<Texture>>,

    // // pub alpha_cutoff: f32,
    // // pub alpha_mode: gltf::material::AlphaMode,

    // // pub double_sided: bool,

}


pub fn shader_flags(
    material: Arc<Mutex<Material>>,
)
-> ShaderFlags
{
    let mut flags = ShaderFlags::empty();
    if material.lock().unwrap().base_color_texture.is_some() {
        flags |= ShaderFlags::HAS_BASECOLORMAP;
    }
    // if material.normal_texture.is_some() {
    //     flags |= ShaderFlags::HAS_NORMALMAP;
    // }
    // if material.emissive_texture.is_some() {
    //     flags |= ShaderFlags::HAS_EMISSIVEMAP;
    // }
    // if material.metallic_roughness_texture.is_some() {
    //     flags |= ShaderFlags::HAS_METALROUGHNESSMAP;
    // }
    // if material.occlusion_texture.is_some() {
    //     flags |= ShaderFlags::HAS_OCCLUSIONMAP;
    // }
    flags
}


pub fn create_material
(
    g_material: gltf::material::Material,
    root: Arc<Mutex<Root>>,
)
-> Material
{
    Material {
        index: Some(0),
        base_color_texture: None,
    }
    
}
