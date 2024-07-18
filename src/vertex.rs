extern crate nalgebra_glm as glm;

pub type Vertex = glm::Vec3;

pub fn new_vertex(x: f32, y: f32, z: f32) -> Vertex {
    glm::vec3(x, y, z)
}
