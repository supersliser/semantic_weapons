use std::ptr::null_mut;

use super::vertex::*;

#[derive(PartialEq, Clone, Copy)]
pub struct HalfEdge {
    pub vert_a: *const Vertex,
    pub vert_b: *const Vertex,
    pub next: *const HalfEdge
}