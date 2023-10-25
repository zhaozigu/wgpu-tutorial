## wgpu learning project

- [ch01_test01](examples/ch01/test01.rs): Initialize blank window
- [ch02_first_triangle](examples/ch02/first_triangle.rs): Hello, Triangle!
- [ch02_color_triangle](examples/ch02/triangle_vertex_color.rs): Triangle with vertex color
- [ch03_point_line](examples/ch03/point_line.rs): draw only a few basic shapes
- [ch03_triangles](examples/ch03/triangles.rs): `TriangleList` and `TriangleStrip`
- [ch04_triangles](examples/ch04/triangles.rs): colored triangle created using GPU buffers
- [ch04_square](examples/ch04/square.rs): colorful square created using GPU buffers
- [ch04_square_index](examples/ch04/square_index.rs): square created using an index GPU buffers
- ch05 is some basic CG mathematical calculations
- [ch06_line3d](examples/ch06/line3d.rs): 3D line
- [ch06_cube_face_color](examples/ch06/cube_face_color.rs): 3D cube
- [ch06_cube_vertex_color](examples/ch06/cube_vertex_color.rs): 3D cube (index buffer)
- [ch06_rotate_cube](examples/ch06/rotate_cube.rs): rotating cube (`update`)
- [ch06_camera_control](examples/ch06/camera_control.rs): interact with the cube using mouse
- [ch07_cube](examples/ch07/cube.rs): cube wireframe
- [ch07_sphere](examples/ch07/sphere.rs): sphere wireframe

## Usage

According to the `[[example]]` in `Cargo.toml`, you can start the corresponding example.

```shell
cargo run --example ch03_point_line
```

## Reference

- Jack Xu, _Practical GPU Graphics with wgpu and Rust_
- [WebGPU Shading Language](https://www.w3.org/TR/WGSL/)
