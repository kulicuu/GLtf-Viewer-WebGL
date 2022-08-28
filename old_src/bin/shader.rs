use bitflags::bitflags;


use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use std::sync::Arc;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use gloo_console::log;
use wasm_bindgen::prelude::*;

use cgmath::{Matrix, Matrix4, Vector3, Vector4};
use cgmath::prelude::*;



pub struct Shader {
    pub id: u32,
    uniform_location_cache: HashMap<&'static str, i32>
}

bitflags! {
    /// Flags matching the defines in the PBR shader
    pub struct ShaderFlags: u16 {
        // vertex shader + fragment shader
        const HAS_NORMALS           = 1;
        const HAS_TANGENTS          = 1 << 1;
        const HAS_UV                = 1 << 2;
        const HAS_COLORS            = 1 << 3;

        // fragment shader only
        const USE_IBL               = 1 << 4;
        const HAS_BASECOLORMAP      = 1 << 5;
        const HAS_NORMALMAP         = 1 << 6;
        const HAS_EMISSIVEMAP       = 1 << 7;
        const HAS_METALROUGHNESSMAP = 1 << 8;
        const HAS_OCCLUSIONMAP      = 1 << 9;
        const USE_TEX_LOD           = 1 << 10;
    }
}

pub struct PbrShader {
    pub shader: Arc<WebGlProgram>,
    // pub shader: WebGlProgram,
    pub flags: ShaderFlags,
    pub uniform_locations: PbrUniformLocations,
}

#[allow(non_snake_case)]
pub struct PbrUniformLocations {
    // pub u_MVPMatrix:  WebGlUniformLocation,
    // pub u_ModelMatrix: WebGlUniformLocation,
    // pub u_Camera: WebGlUniformLocation,

    // pub u_LightDirection: WebGlUniformLocation,
    // pub u_LightColor: WebGlUniformLocation,

    // pub u_AmbientLightColor: WebGlUniformLocation,
    // pub u_AmbientLightIntensity: WebGlUniformLocation,

    // // // TODO!: set when integrating IBL (unused now)
    // pub u_DiffuseEnvSampler: WebGlUniformLocation,
    // pub u_SpecularEnvSampler: WebGlUniformLocation,
    // pub u_brdfLUT: WebGlUniformLocation,

    // pub u_BaseColorSampler: WebGlUniformLocation,
    // pub u_BaseColorFactor: WebGlUniformLocation,

    // pub u_NormalSampler: WebGlUniformLocation,
    // pub u_NormalScale: WebGlUniformLocation,

    // pub u_EmissiveSampler: WebGlUniformLocation,
    // pub u_EmissiveFactor: WebGlUniformLocation,

    // pub u_MetallicRoughnessSampler: WebGlUniformLocation,
    // pub u_MetallicRoughnessValues: WebGlUniformLocation,

    // pub u_OcclusionSampler: WebGlUniformLocation,
    // pub u_OcclusionStrength: WebGlUniformLocation,

    // // // TODO!: use/remove debugging uniforms
    // // // debugging flags used for shader output of intermediate PBR variables
    // pub u_ScaleDiffBaseMR: WebGlUniformLocation,
    // pub u_ScaleFGDSpec: WebGlUniformLocation,
    // pub u_ScaleIBLAmbient: WebGlUniformLocation,
}



impl PbrShader {
    pub fn new(
        gl: Arc<GL>,
        flags: ShaderFlags,
    )
    -> Self 
    {
        // doing some
        log!("Creating PbrShader");

        let vert_code = include_str!("../shaders/pbr-vert.glsl");
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);
        let vert_shader_log = gl.get_shader_info_log(&vert_shader);
        log!("pbr-vert.glsl compilation log: ", vert_shader_log);
    
        let frag_code = include_str!("../shaders/pbr-frag.glsl");
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);
        let frag_shader_log = gl.get_shader_info_log(&frag_shader);
        log!("pbr-frag.glsl compilation log: ", frag_shader_log);
    
        let shader = Arc::new(gl.create_program().unwrap());
        gl.attach_shader(&shader, &vert_shader);
        gl.attach_shader(&shader, &frag_shader);
        gl.link_program(&shader);
    
        let x = gl.get_uniform_location(&shader, "u_MVPMatrix").unwrap();
    
        
        
        // let mut shader = Shader::from_source(
        //     include_str!("shaders/pbr-vert.glsl"),
        //     include_str!("shaders/pbr-frag.glsl"),
        //     &flags.as_strings());

        // // NOTE: shader debug version
        // // let mut shader = Shader::new(
        // //     "src/shaders/pbr-vert.glsl",
        // //     "src/shaders/pbr-frag.glsl",
        // //     &flags.as_strings());


        // Let's see what we need to reconstruct:
        // We need 

        

        
        let uniform_locations = PbrUniformLocations {
            // u_MVPMatrix: gl.get_uniform_location(&shader, "u_MVPMatrix").unwrap(),
            // u_ModelMatrix: gl.get_uniform_location(&shader, "u_ModelMatrix").unwrap(),
            // u_Camera: gl.get_uniform_location(&shader, "u_Camera").unwrap(),
            // u_LightDirection: gl.get_uniform_location(&shader, "u_LightDirection").unwrap(),
            // u_LightColor: gl.get_uniform_location(&shader, "u_LightColor").unwrap(),
            // u_AmbientLightColor: gl.get_uniform_location(&shader, "u_AmbientLightColor").unwrap(),
            // u_AmbientLightIntensity: gl.get_uniform_location(&shader, " u_AmbientLightIntensity").unwrap(),
            // u_DiffuseEnvSampler: gl.get_uniform_location(&shader, "u_DiffuseEnvSampler").unwrap(),
            // u_SpecularEnvSampler: gl.get_uniform_location(&shader, "u_SpecularEnvSampler").unwrap(),
            // u_brdfLUT: gl.get_uniform_location(&shader, "u_brdfLUT").unwrap(),
            // u_BaseColorSampler: gl.get_uniform_location(&shader, "u_BaseColorSampler").unwrap(),
            // u_BaseColorFactor: gl.get_uniform_location(&shader, "u_BaseColorFactor").unwrap(),
            // u_NormalSampler: gl.get_uniform_location(&shader, "u_NormalSampler").unwrap(),
            // u_NormalScale: gl.get_uniform_location(&shader, "u_NormalScale").unwrap(),
            // u_EmissiveSampler: gl.get_uniform_location(&shader, "u_EmissiveSampler").unwrap(),
            // u_EmissiveFactor: gl.get_uniform_location(&shader, " u_EmissiveFactor").unwrap(),
            // u_MetallicRoughnessSampler: gl.get_uniform_location(&shader, "u_MetallicRoughnessSampler").unwrap(),
            // u_MetallicRoughnessValues: gl.get_uniform_location(&shader, "u_MetallicRoughnessValues").unwrap(),
            // u_OcclusionSampler: gl.get_uniform_location(&shader, "u_OcclusionSampler").unwrap(),
            // u_OcclusionStrength: gl.get_uniform_location(&shader, "u_OcclusionStrength").unwrap(),
            // u_ScaleDiffBaseMR: gl.get_uniform_location(&shader, "u_ScaleDiffBaseMR").unwrap(),
            // u_ScaleFGDSpec: gl.get_uniform_location(&shader, " u_ScaleFGDSpec").unwrap(),
            // u_ScaleIBLAmbient: gl.get_uniform_location(&shader, "u_ScaleIBLAmbient").unwrap(),
        };

            // shader.use_program();
            // shader.set_int(uniforms.u_BaseColorSampler, 0);
            // shader.set_int(uniforms.u_NormalSampler, 1);
            // shader.set_int(uniforms.u_EmissiveSampler, 2);
            // shader.set_int(uniforms.u_MetallicRoughnessSampler, 3);
            // shader.set_int(uniforms.u_OcclusionSampler, 4);

            // shader.set_vec3(uniforms.u_LightColor, 5.0, 5.0, 5.0);
            // // TODO!: optional minus on z
            // shader.set_vec3(uniforms.u_LightDirection, 0.0, 0.5, 0.5);

            // shader.set_vec3(uniforms.u_AmbientLightColor, 1.0, 1.0, 1.0);
            // shader.set_float(uniforms.u_AmbientLightIntensity, 0.2);



        Self {
            shader,
            flags,
            uniform_locations,
        }
    }
}
