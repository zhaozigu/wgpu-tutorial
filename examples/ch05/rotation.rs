use cgmath::{Matrix4, Rad, Vector4};
use std::f32::consts::PI;

fn main() {
    let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);
    let rot_mat_z = Matrix4::from_angle_z(Rad(20.0 * PI / 180.0));
    let rot_mat = rot_mat_z * Matrix4::from_angle_z(Rad(25.0 * PI / 180.0));

    let rot_vec = rot_mat * my_vec;

    println!("\nOriginal vector: my_vec = \n{:?}", my_vec);
    println!(
        "Total roation matrix after two rotaitons: rot_mat = \n{:?}",
        rot_mat
    );
    println!(
        "Vector after two rotations: rot_vec = rot_mat * my_vec = \n{:?}\n",
        rot_vec
    );
}
