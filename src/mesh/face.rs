use core::prelude::v1;
use std::{fmt::Pointer, ptr::null};

use glam::Vec3;

use super::vertex::*;
use crate::mesh::{
    Mesh,
    half_edge::{self, HalfEdge},
};

impl Mesh {
    fn get_directed_half_edges(&self) -> Vec<HalfEdge> {
        let mut output = Vec::new();
        for vert_a in 0..self.vertices.len() {
            let mut vertex_edges = vec![self.get_vertex(vert_a).unwrap()];
            for vert_b in 0..self.vertices.len() {
                if vert_b == vert_a {
                    continue;
                }
                if self.are_connected(vert_a, vert_b) {
                    vertex_edges.push(self.get_vertex(vert_b).unwrap());
                }
            }
            let mut angles = Vec::new();
            for v in vertex_edges.as_slice() {
                let edge_vector = vertex_edges[0].get_position() - v.get_position();
                let x = edge_vector.dot(vertex_edges[0].get_position());
                let y = edge_vector.dot(v.get_position());
                let theta = x.atan2(y);
                angles.push((v, theta));
            }
            for i in 0..angles.len() {
                for j in 0..angles.len() - 1 {
                    if angles[i].1 > angles[j].1 {
                        let temp = angles[j];
                        angles[j] = angles[i];
                        angles[i] = angles[j]
                    }
                }
            }
            for vertex in angles {
                output.push(HalfEdge {
                    vert_a: self.get_vertex(vert_a).unwrap(),
                    vert_b: *vertex.0,
                    next: null(),
                });
                let prev_index = output.len() - 1;
                output[prev_index].next = output.last().unwrap();
            }
        }
        output
    }

    pub fn get_faces(&self) -> Vec<Vec<&Vertex>> {
        let mut visited = Vec::new();
        let mut output = Vec::new();

        let half_edges = self.get_directed_half_edges();

        for edge in half_edges {
            if visited.contains(&edge) {
                continue;
            }
            let mut face = Vec::new();

            let mut current_edge = edge;

            unsafe {
                loop {
                    face.push(&*current_edge.vert_a);
                    visited.push(current_edge);

                    current_edge = *current_edge.next;

                    if current_edge == edge {
                        break;
                    }
                }
            }
            output.push(face);
        }

        output
    }

    pub fn get_face_normal(&self, face: Vec<&Vertex>) -> Vec3 {
        let mut total_normal = Vec3::ZERO;
        for v in face.as_slice() {
            total_normal += v.get_normal();
        }
        let len = face.len() as f32;
        total_normal / Vec3 {x: len, y: len, z: len }
    }
}
