use super::vertex::*;

pub mod edit_mesh;

pub struct Mesh {
    vertices: Vec<Vertex>,
    connections: Vec<bool>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            connections: Vec::new(),
        }
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Option<Vertex>) {
        if let Some(v) = vertex {
            self.vertices.push(v);
        }
        else {
            self.vertices.push(Vertex::default());
        }
        self.connections
            .extend(vec![false; self.vertices.len().pow(2) - 1]);
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_vertices_positions(&self) -> Vec<glam::Vec3> {
        self.vertices.iter().map(|v| v.get_position()).collect()
    }

    pub fn get_vertex(&self, index: usize) -> Option<&Vertex> {
        self.vertices.get(index)
    }

    pub fn connect_vertices(&mut self, index_a: usize, index_b: usize) {
        if index_a >= self.vertices.len() || index_b >= self.vertices.len() {
            return;
        }
        let conn_index = index_a * self.vertices.len() + index_b;
        self.connections[conn_index] = true;
    }

    pub fn are_connected(&self, index_a: usize, index_b: usize) -> bool {
        if index_a >= self.vertices.len() || index_b >= self.vertices.len() {
            return false;
        }
        let conn_index = index_a * self.vertices.len() + index_b;
        self.connections[conn_index]
    }

    pub fn disconnect_vertices(&mut self, index_a: usize, index_b: usize) {
        if index_a >= self.vertices.len() || index_b >= self.vertices.len() {
            return;
        }
        let conn_index = index_a * self.vertices.len() + index_b;
        self.connections[conn_index] = false;
    }
}