use glam::Vec3;

use super::vertex::*;
use crate::mesh::{Mesh, half_edge::HalfEdge};

impl Mesh {
    pub fn get_directed_half_edges(&self, up: Vec3, right: Vec3, fwd: Vec3) -> Vec<HalfEdge> {
        let mut output = Vec::new();
        let mut directed_edges = Vec::new();
        for vert_a in 0..self.vertices.len() {
            let mut vertex_edges = vec![self.get_vertex(vert_a).unwrap()];
            for vert_b in 0..self.vertices.len() {
                if vert_b == vert_a {
                    continue;
                }
                if self.are_connected( vert_a, vert_b) {
                    vertex_edges.push(self.get_vertex(vert_b).unwrap());
                }
            }
            let angles = Vec::new();
            for v in vertex_edges {
                let edge_vector = vertex_edges[0].get_position() - v.get_position();
                let x = edge_vector.dot(u.get_position());
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
            let adjacent_angles = Vec::new();
            for vertex in angles {
                adjacent_angles.push((vert_a, vertex.0))
            }
            output.push(adjacent_angles)
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

            loop {
                face.add(current_edge.0);
                visited.add(current_edge);
                current_edge = edge.
                if current_edge == edge {
                    break;
                }
            } 
        } 

        output
    }
}