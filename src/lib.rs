use std::vec;
use bytemuck::NoUninit;
use core_2d::{pipeline_2d::Pipeline2D, drawable_2d::{Drawable2D, SimpleDrawable2D}, box_2d::Box2D, drawable_state_2d::Anchor};
use math::color::Color;
use wgpu::{Limits, DepthStencilState, SurfaceConfiguration, Surface, Device, RenderPipeline, Queue, ShaderModule, PipelineLayout, TextureFormat, ColorTargetState, util::{DeviceExt, BufferInitDescriptor}, VertexBufferLayout, Buffer, BindGroupLayout, BindGroup, BufferUsages, RenderPass, CommandEncoder, SurfaceTexture, TextureView};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::{PhysicalSize, Size},
};

pub mod renderer;
pub mod core_2d;
pub mod math;
use renderer::{ShaderModules, WgpuState};

pub struct Game {
    ui_container: SimpleDrawable2D,
    pipeline_2d: Pipeline2D
}

impl Game {
    pub fn prepare(r_state: &mut WgpuState) -> Game {
        let mut ui_container = SimpleDrawable2D::new();
        ui_container.set_abs_pos(100.0, 100.0);
        ui_container.set_abs_size(500.0, 500.0);
        let mut test = Box2D::new();
        //test.set_rel_size(0.5, 0.5);
        test.set_abs_size(300.0, 300.0);
        test.set_color(Color::new(186, 26, 66, 255));
        ui_container.add_child(Box::new(test));

        let mut test2 = Box2D::new();
        test2.set_abs_size(300.0, 300.0);
        //test2.set_rel_pos(0.5, 0.5);
        test2.set_alignment(Anchor::TOP_LEFT);
        test2.set_origin(Anchor::BOTTOM_RIGHT);
        test2.set_color(Color::new(12, 150, 67, 255));
        ui_container.add_child(Box::new(test2));

        let pipeline_2d = Pipeline2D::new(&r_state, &mut ui_container);

        let game = Game {
            ui_container,
            pipeline_2d,
        };
        return game;
    }

    pub fn update(&mut self) {

    }

    pub fn draw<'a>(&'a mut self, r_state: &mut WgpuState, encoder: &mut CommandEncoder, view: &TextureView) {
        self.pipeline_2d.draw(r_state, encoder, view, &mut self.ui_container);
    }

    pub fn on_resized(&mut self, r_state: &mut WgpuState, new_size: PhysicalSize<u32>) {
        r_state.config.width = new_size.width.max(1);
        r_state.config.height = new_size.height.max(1);
        r_state.surface.configure(&r_state.device, &r_state.config);
        r_state.window.request_redraw();
    }
}