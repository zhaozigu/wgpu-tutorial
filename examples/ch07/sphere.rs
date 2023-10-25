use cgmath::*;
mod common;
#[path = "../common/math_func.rs"]
mod math_func;

fn create_vertices(r: f32, u: usize, v: usize) -> Vec<common::Vertex> {
    let mut pts: Vec<common::Vertex> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    for i in 0..u - 1 {
        for j in 0..v - 1 {
            let theta = i as f32 * 180.0 / (u as f32 - 1.0);
            let phi = j as f32 * 360.0 / (v as f32 - 1.0);
            
            let theta1 = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
            let phi1 = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);
            
            let p0 = math_func::sphere_position(r, Deg(theta), Deg(phi));
            let p1 = math_func::sphere_position(r, Deg(theta1), Deg(phi));
            let p3 = math_func::sphere_position(r, Deg(theta), Deg(phi1));

            pts.push(common::vertex(p0));
            pts.push(common::vertex(p1));
            pts.push(common::vertex(p0));
            pts.push(common::vertex(p3));
        }
    }
    pts.to_vec()
}

fn main() {
    let title = "sphere";
    let mesh_data = create_vertices(1.7, 15, 20);
    common::run(&mesh_data, title);
}
