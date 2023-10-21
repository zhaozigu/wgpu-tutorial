use cgmath::*;
use std::f32::consts::FRAC_PI_6;

fn main() {
    let left = -3.0;
    let right = 3.0;
    let bottom = -5.0;
    let top = 5.0;
    let near = 1.0;
    let far = 100.0;
    let fovy = FRAC_PI_6;
    let aspect = 1.5;

    let frustum_mat = frustum(left, right, bottom, top, near, far);
    let persp_mat = perspective(Rad(fovy), aspect, near, far);

    println!("\nfrustum matrix: {:?}\n", frustum_mat);
    println!("perspective matrix: {:?}\n", persp_mat);

    let ortho_mat = ortho(left, right, bottom, top, near, far);
    println!("orthographic matrix: {:?}\n", ortho_mat);
}
