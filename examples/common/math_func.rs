use ::cgmath::*;

#[allow(dead_code)]
pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
    let snt = theta.sin();
    let cnt = theta.cos();
    let snp = phi.sin();
    let cnp = phi.cos();

    [r * snt * cnp, r * cnt, -r * snt * snp]
}

#[allow(dead_code)]
pub fn cylinder_position(r: f32, y: f32, theta: Deg<f32>) -> [f32; 3] {
    [r * theta.cos(), y, -r * theta.sin()]
}

#[allow(dead_code)]
pub fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> [f32; 3] {
    let x = (r_torus + r_tube * v.cos()) * u.cos();
    let y = r_tube * v.sin();
    let z = -(r_torus + r_tube * v.cos()) * u.sin();
    
    [x, y, z]
}
