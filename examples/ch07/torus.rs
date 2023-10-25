use cgmath::*;
mod common;
#[path = "../common/math_func.rs"]
mod math_func;

fn create_vertices(
    r_torus: f32,
    r_tube: f32,
    n_torus: usize,
    n_tube: usize,
) -> Vec<common::Vertex> {
    let mut pts: Vec<common::Vertex> =
        Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);
    for i in 0..n_torus - 1 {
        for j in 0..n_tube - 1 {
            let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
            let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);

            let u1 = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
            let v1 = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);

            let p0 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v));
            let p1 = math_func::torus_position(r_torus, r_tube, Deg(u1), Deg(v));
            let p3 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v1));

            pts.push(common::vertex(p0));
            pts.push(common::vertex(p1));
            pts.push(common::vertex(p0));
            pts.push(common::vertex(p3));
        }
    }
    pts.to_vec()
}

fn main() {
    let title = "torus";
    let mesh_data = create_vertices(1.5, 0.3, 40, 13);
    common::run(&mesh_data, title);
}
