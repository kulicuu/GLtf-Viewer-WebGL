#![macro_use]

use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use gloo_console::log;

use std::sync::Arc;

use crate::shader::*;
use crate::render::mesh::Mesh;
use crate::render::node::Node;
use crate::render::texture::Texture;
use crate::render::material::Material;
use crate::shader::{ShaderFlags};
// use crate::import_data::ImportData;
use crate::render::texture::ImportData;


pub struct Root {
    pub nodes: Vec<Node>,
    pub meshes: Vec<Rc<Mesh>>, 
    pub textures: Vec<Rc<Texture>>,
    pub materials: Vec<Rc<Material>>,
    pub shaders: HashMap<ShaderFlags, Arc<PbrShader>>,

    pub camera_nodes: Vec<usize>, // indices of camera nodes
    // TODO!: joint_nodes, mesh_nodes?
}

impl Root {
    pub fn from_gltf(gl: Arc<GL>, imp: &ImportData) -> Self {
        // let mut root = Root::default();
        let mut root = Root {
            nodes: vec![],
            meshes: vec![],
            textures: vec![],
            materials: vec![],
            shaders: HashMap::new(),
            camera_nodes: vec![],
        };

        log!("creating root");
        // log!("imp.doc.nodes.len()", imp.doc.nodes().len());
        // let ints: std::vec::Vec<u8> = imp.doc.nodes()
        //     .map(|g_node| {
        //         1
        //     }).collect();
        // log!("ints[0]", ints[0]);
        // for g_node in imp.doc.nodes() {
        //     log!("g_node", g_node.index());
        //     // g_node.camera().unwrap();
        //     // log!("g_node", g_node.camera().unwrap());
        //     log!("children", g_node.children().len());
        //     g_node.mesh().unwrap().primitives();
        //     log!("primitives len", g_node.mesh().unwrap().primitives().len());
        //     log!("one");
        // }

        let nodes: Vec<Node> = imp.doc.nodes()
            .map(|g_node| Node::from_gltf(gl.clone(), &g_node, &mut root, imp))
            .collect();
        root.nodes = nodes;
        root.camera_nodes = root.nodes.iter()
            .filter(|node| node.camera.is_some())
            .map(|node| node.index)
            .collect();
        root
    }

    /// Get a mutable reference to a node without borrowing `Self` or `Self::nodes`.
    /// Safe for tree traversal (visiting each node ONCE and NOT keeping a reference)
    /// as long as the gltf is valid, i.e. the scene actually is a tree.
    pub fn unsafe_get_node_mut(&mut self, index: usize) ->&'static mut Node {
        unsafe {
            &mut *(&mut self.nodes[index] as *mut Node)
        }
    }

    /// Note: index refers to the vec of camera node indices!
    pub fn get_camera_node(&self, index: usize) -> &Node {
        &self.nodes[self.camera_nodes[index]]
    }
}
