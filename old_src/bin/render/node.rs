use std::rc::Rc;
use std::path::Path;

use gltf;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};


use gloo_console::log;

use std::sync::Arc;

use collision::{Aabb, Union};

use crate::controls::CameraParams;
use crate::render::math::*;
use crate::render::mesh::Mesh;
use crate::render::root::Root;
use crate::render::camera::Camera;
// use crate::import_data::ImportData;
use crate::render::texture::ImportData;





pub struct Node {
    pub index: usize, // glTF index
    pub children: Vec<usize>,
    pub matrix: Matrix4,
    pub mesh: Option<Rc<Mesh>>,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub translation: Vector3,
    // TODO: weights
    // weights_id: usize,
    pub camera: Option<Camera>,
    pub name: Option<String>,

    pub final_transform: Matrix4, // including parent transforms
    // pub bounds: Aabb3,
}

use cgmath;


impl Node {
    // TODO!: refactor transformations using mint and non-deprecated functions
    pub fn from_gltf(
        gl: Arc<GL>,
        g_node: &gltf::Node,
        root: &mut Root,
        imp: &ImportData,

    ) -> Node {
        // convert matrix in 3 steps due to type system weirdness

        log!("creating node");


        let matrix = &g_node.transform().matrix();
        let matrix: &Matrix4 = matrix.into();
        let matrix = *matrix;

        let (trans, rot, scale) = g_node.transform().decomposed();
        let r = rot;
        log!("rot[1]:", rot[1]);
        let rotation = Quaternion::new(r[3], r[0], r[1], r[2]); // NOTE: different element order!
        log!("scale: ", scale[1]);
        let mut mesh = None;
        if let Some(g_mesh) = g_node.mesh() {
            if let Some(existing_mesh) = root.meshes.iter().find(|mesh| (***mesh).index == g_mesh.index()) {
                log!("existing mesh");
                mesh = Some(Rc::clone(existing_mesh));
            }

            if mesh.is_none() { // not using else due to borrow-checking madness
                log!("mesh.is_none()");
                mesh = Some(Rc::new(Mesh::from_gltf(
                    gl.clone(),
                    &g_mesh, root, imp)));
                root.meshes.push(mesh.clone().unwrap());
            }
        }
        let children: Vec<_> = g_node.children()
                .map(|g_node| g_node.index())
                .collect();

        Node {
            index: g_node.index(),
            children,
            matrix,
            mesh,
            rotation,
            scale: scale.into(),
            translation: trans.into(),
            camera: g_node.camera().as_ref().map(Camera::from_gltf),
            name: g_node.name().map(|s| s.into()),

            final_transform: Matrix4::identity(),

            // bounds: Aabb3::new(
            //     cgmath::Point3::new(0.0, 0.0, 0.0), 
            //     cgmath::Point3::new(0.0, 0.0, 0.0),
            // ),
        }
    }

    // pub fn update_transform(&mut self, root: &mut Root, parent_transform: &Matrix4) {
    //     self.final_transform = *parent_transform;

    //     if !self.matrix.is_identity() {
    //         self.final_transform = self.final_transform * self.matrix;
    //     }
    //     else {
    //         // TODO?: detect if all default and set None? does NOT happen for any sample model
    //         self.final_transform = self.final_transform *
    //             Matrix4::from_translation(self.translation) *
    //             Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) *
    //             Matrix4::from(self.rotation);
    //     }

    //     for node_id in &self.children {
    //         let node = root.unsafe_get_node_mut(*node_id);
    //         node.update_transform(root, &self.final_transform);
    //     }
    // }


    // pub fn update_bounds(&mut self, root: &mut Root) {
    //     self.bounds = Aabb3::zero();
    //     if let Some(ref mesh) = self.mesh {
    //         self.bounds = mesh.bounds
    //             .transform(&self.final_transform);
    //     }
    //     else if self.children.is_empty() {
    //         // Cameras (others?) have neither mesh nor children. Their position is the origin
    //         // TODO!: are there other cases? Do bounds matter for cameras?
    //         self.bounds = Aabb3::zero();
    //         self.bounds = self.bounds.transform(&self.final_transform);
    //     }
    //     else {
    //         for node_id in &self.children {
    //             let node = root.unsafe_get_node_mut(*node_id);
    //             node.update_bounds(root);
    //             self.bounds = self.bounds::Union.union(&node.bounds);
    //         }
    //     }
    // }

    pub fn draw(
        &mut self, 
        gl: Arc<GL>,
        root: &mut Root, 
        cam_params: &CameraParams,
    ) {

        // log!("Node Draw");
        if let Some(ref mesh) = self.mesh {
            let mvp_matrix = cam_params.projection_matrix * cam_params.view_matrix * self.final_transform;

            (*mesh).draw(
                gl.clone(),
                &self.final_transform, 
                &mvp_matrix, 
                &cam_params.position,
            );
        }
        for node_id in &self.children {
            let node = root.unsafe_get_node_mut(*node_id);
            node.draw(gl.clone(), root, cam_params);
        }
    }
}
