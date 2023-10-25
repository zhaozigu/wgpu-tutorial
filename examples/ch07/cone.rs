use cgmath::*;
mod common;
#[path = "../common/math_func.rs"]
mod math_func;

fn create_vertices(rtop: f32, rbottom: f32, height: f32, n: usize) -> Vec<common::Vertex> {
    let h = height / 2.0;
    let mut pts: Vec<common::Vertex> = Vec::with_capacity(10 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rtop, h, Deg(theta));
        let p1 = math_func::cylinder_position(rbottom, -h, Deg(theta));
        let p2 = math_func::cylinder_position(0.0, -h, Deg(theta));
        let p3 = math_func::cylinder_position(0.0, h, Deg(theta));
        let p4 = math_func::cylinder_position(rtop, h, Deg(theta1));
        let p5 = math_func::cylinder_position(rbottom, -h, Deg(theta1));

        // top face 2 lines
        pts.push(common::vertex(p0));
        pts.push(common::vertex(p3));
        pts.push(common::vertex(p4));
        pts.push(common::vertex(p0));
        // bottom face 2 lines
        pts.push(common::vertex(p1));
        pts.push(common::vertex(p2));
        pts.push(common::vertex(p5));
        pts.push(common::vertex(p1));
        // side 1 line
        pts.push(common::vertex(p0));
        pts.push(common::vertex(p1));
    }
    pts.to_vec()
}

fn main() {
    let title = "cone";
    let mesh_data = create_vertices(0.0, 1.0, 2.0, 20);
    common::run(&mesh_data, title);
}
