use::cgmath::*;

pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
    let snt = theta.sin();
    let cnt = theta.cos();
    let snp = phi.sin();
    let cnp = phi.cos();

    [r * snt * cnp, r * cnt, -r * snt * snp]
}
