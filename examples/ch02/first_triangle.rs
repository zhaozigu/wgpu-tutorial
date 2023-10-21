#[path = "../common/common.rs"]
mod common;

use std::borrow::Cow;
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("first_triangle");
    env_logger::init();

    let inputs = common::Inputs {
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("first_triangle.wgsl"))),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
    };

    pollster::block_on(common::run(event_loop, window, inputs, 3));
}
