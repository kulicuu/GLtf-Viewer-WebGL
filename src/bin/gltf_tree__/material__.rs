use gltf;

use std::sync::{Arc, Mutex};

use crate::gltf_tree__::root__::Root;

use gloo_console::log;


pub struct Material {
    // gl: Arc<GL>,
    pub index: Option<usize>, 
    // pub name: Option<String>,

    // // pbr_metallic_roughness properties
    // // pub base_color_factor: Vector4,
    // pub base_color_texture: Option<Rc<Texture>>,
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

pub fn create_material
(
    g_material: gltf::material::Material,
    root: Arc<Mutex<Root>>,
)
-> Material
{
    Material {
        index: Some(0),
    }
    
}
