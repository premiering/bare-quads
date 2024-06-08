use std::{vec, ptr::null};
use bytemuck::NoUninit;
use wgpu::{Limits, DepthStencilState, SurfaceConfiguration, Surface, Device, RenderPipeline, Queue, ShaderModule, PipelineLayout, TextureFormat, ColorTargetState, util::{DeviceExt, BufferInitDescriptor}, VertexBufferLayout, Buffer, BindGroupLayout, BindGroup, BufferUsages, RenderPass, CommandEncoder, SurfaceTexture, TextureView};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::{PhysicalSize, Size},
};
use rustyfun::{Game, renderer::{WgpuState, DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT}};

fn main() {
    unsafe {
        pollster::block_on(run());
    }
}

async unsafe fn run() {
    // Set everything up so that we can create a WgpuState
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_inner_size(PhysicalSize::new(DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT)).build(&event_loop).unwrap();

    let mut size = window.inner_size();
    size.width = size.width.min(1);
    size.height = size.height.max(1);

    let mut r_state = pollster::block_on(WgpuState::new(window));
    
    //WgpuState is created, initialize the game.
    let mut game = Game::prepare(&mut r_state);

    //Run the game...
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
    
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    game.on_resized(&mut r_state, new_size);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                r_state.window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                let current_frame = r_state.surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = current_frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut current_encoder = r_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: None,
                });
                game.draw(&mut r_state, &mut current_encoder, &view);
                r_state.queue.submit(Some(current_encoder.finish()));
                current_frame.present();
            }
            _ => (),
        }
    });
}