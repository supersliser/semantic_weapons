use super::vertex::*;
pub struct HalfEdge {
    pub vert_a: *const Vertex,
    pub vert_b: *const Vertex,
    pub next: *const HalfEdge
}