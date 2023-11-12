mod common;
#[path = "../common/vertex_data.rs"]
mod vertex_data;

fn vertex(p: [i8; 3], n: [i8; 3]) -> common::Vertex {
    common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
    }
}

fn create_vertices() -> Vec<common::Vertex> {
    let (pos, _col, _uv, normal) = vertex_data::cube_data();
    let mut data: Vec<common::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

fn main() {
    let vertex_data = create_vertices();
    let light_data = common::light(
        [1.0, 0.0, 0.0], // Color
        [1.0, 1.0, 0.0], // Specular Color
        0.1,             // Ambient Intensity
        0.6,             // Diffuse Intensity
        0.3,             // Specular Intensity
        30.0,            // Specular Shininess
    );
    common::run(&vertex_data, light_data, "cube");
}
