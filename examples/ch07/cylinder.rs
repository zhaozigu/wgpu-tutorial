use cgmath::*;
mod common;
#[path = "../common/math_func.rs"]
mod math_func;

fn create_vertices(rin: f32, rout: f32, height: f32, n: usize) -> Vec<common::Vertex> {
    let h = height / 2.0;
    let mut pts: Vec<common::Vertex> = Vec::with_capacity(16 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rout, h, Deg(theta));
        let p1 = math_func::cylinder_position(rout, -h, Deg(theta));
        let p2 = math_func::cylinder_position(rin, -h, Deg(theta));
        let p3 = math_func::cylinder_position(rin, h, Deg(theta));
        let p4 = math_func::cylinder_position(rout, h, Deg(theta1));
        let p5 = math_func::cylinder_position(rout, -h, Deg(theta1));
        let p6 = math_func::cylinder_position(rin, -h, Deg(theta1));
        let p7 = math_func::cylinder_position(rin, h, Deg(theta1));

        // top face 3 lines
        pts.push(common::vertex(p0));
        pts.push(common::vertex(p3));
        pts.push(common::vertex(p3));
        pts.push(common::vertex(p7));
        pts.push(common::vertex(p4));
        pts.push(common::vertex(p0));

        // bottom face 3 lines
        pts.push(common::vertex(p1));
        pts.push(common::vertex(p2));
        pts.push(common::vertex(p2));
        pts.push(common::vertex(p6));
        pts.push(common::vertex(p5));
        pts.push(common::vertex(p1));

        // side 2 lines
        pts.push(common::vertex(p0));
        pts.push(common::vertex(p1));
        pts.push(common::vertex(p3));
        pts.push(common::vertex(p2));
    }
    pts.to_vec()
}

fn main() {
    let title = "cylinder";
    let mesh_data = create_vertices(0.5, 1.0, 2.5, 50);
    common::run(&mesh_data, title);
}
