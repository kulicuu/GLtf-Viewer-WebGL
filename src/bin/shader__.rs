
use bitflags::bitflags;
use std::sync::{Arc, Mutex};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


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