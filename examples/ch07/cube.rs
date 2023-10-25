mod common;

fn create_vertices() -> Vec<common::Vertex> {
    // vertex positions
    let p: [[f32; 3]; 8] = [
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
    ];
    // line segments
    let lines: [[f32; 3]; 24] = [
        // 4 lines on top face
        p[0], p[1], p[1], p[2], p[2], p[3], p[3], p[0], // 4 lines on bottom face
        p[4], p[5], p[5], p[6], p[6], p[7], p[7], p[4], // 4 lines on sides
        p[0], p[4], p[1], p[5], p[2], p[6], p[3], p[7],
    ];
    let mut data: Vec<common::Vertex> = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        data.push(common::vertex(lines[i]));
    }
    data.to_vec()
}

fn main() {
    let title = "cube";
    let mesh_data = create_vertices();
    common::run(&mesh_data, title);
}
