use std::{sync::Arc, rc::Rc, cell::RefCell, marker::PhantomData, collections::HashMap};

use bytemuck::{Pod, Zeroable};
use wgpu::{RenderPipeline, BindGroup, Device, RenderPass, ColorTargetState, BindGroupLayout, Buffer, util::{BufferInitDescriptor, DeviceExt, StagingBelt}, BufferUsages, TextureFormat, MultisampleState, CommandEncoder, TextureView, BufferSlice};
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrushBuilder, GlyphBrush, Section, Text};

use crate::renderer::{ShaderModules, WgpuState};

use super::{drawable_2d::{Drawable2D, SimpleDrawable2D}, drawable_state_2d::{DrawNode2D, DrawableId, QuadDrawNode2D}};

// Info used by the renderer to render the drawable
struct DrawableRenderInfo {
    nodes: Vec<DrawNode2D>,
    buffers: Vec<Buffer>
}

pub struct Pipeline2D {
    root_id: DrawableId, // Used for API checking.
    pub pipeline_2d: Box<RenderPipeline>,
    quad_index_buffer: Buffer,
    current_buffer: Buffer,
    debug_glyph_brush: GlyphBrush<()>,
    debug_glyph_staging_belt: StagingBelt,
}

impl Pipeline2D {
    pub fn new(r_state: &WgpuState, root: &mut SimpleDrawable2D) -> Pipeline2D {
        let pipeline_layout = r_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
        let pipeline_2d = r_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &r_state.shader_modules.ui_shader,
                entry_point: "vertex",
                buffers: &[
                    UIVertex::describe(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &r_state.shader_modules.ui_shader,
                entry_point: "fragment",
                targets: &[Some(r_state.color_target.clone())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let quad_slice: &[u16] = &[
            0, 1, 2, 2, 3, 0
        ];
        let quad_index_buffer = r_state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Quad Index Buffer"),
            contents: bytemuck::cast_slice(quad_slice),
            usage: BufferUsages::INDEX,
        });

        // Set up font rendering
        // Prepare glyph_brush
        let montserrat = FontArc::try_from_slice(include_bytes!(
            "../Montserrat-Light.ttf"
        ));

        let debug_glyph_brush = GlyphBrushBuilder::using_font(montserrat.expect(""))
            .build(&r_state.device, r_state.swapchain_format);

        return Pipeline2D {
            root_id: root.get_id(),
            pipeline_2d: Box::new(pipeline_2d),
            quad_index_buffer,
            current_buffer: r_state.device.create_buffer_init(&BufferInitDescriptor{label: None, contents: &[], usage: BufferUsages::VERTEX}),
            debug_glyph_brush,
            debug_glyph_staging_belt: StagingBelt::new(1024),
        };
    }

    pub fn draw<'a>(&'a mut self, r_state: &mut WgpuState, encoder: &mut CommandEncoder, view: &TextureView, root: &mut dyn Drawable2D) {
        //let pipeline_2d = &self.pipeline_2d;
        if root.get_id() != self.root_id {
            panic!("Pipeline2D::draw was called using a different container than expected!");
        }
        let window_size = &r_state.get_size();
        let mut rpass_quad: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass_quad.set_pipeline(self.pipeline_2d.as_ref());
        let mut rpass_text: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        let draw_result = root.draw();
        let mut current_draws: Vec<DrawNode2D> = if draw_result.is_some() { draw_result.expect("") } else { vec![] };

        let mut current_buffer: &'a mut Buffer;
        {
            for node in &mut current_draws {
                match node {
                    DrawNode2D::Quad(quad) => {
                        let window_size = &r_state.get_size();
                        let mut vbuf: [UIVertex; 4] = [
                            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x, quad.quad.abs_pos.y, 1.0]},
                            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x + quad.quad.abs_size.x, quad.quad.abs_pos.y, 1.0]},
                            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x + quad.quad.abs_size.x, quad.quad.abs_pos.y + quad.quad.abs_size.y, 1.0]},
                            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x, quad.quad.abs_pos.y + quad.quad.abs_size.y, 1.0]}
                        ];
                        for mut ele in &mut vbuf {
                            ele.pos[0] = (ele.pos[0] / (window_size.x as f32)) * 2.0 - 1.0;
                            ele.pos[1] = -((ele.pos[1] / (window_size.y as f32)) * 2.0 - 1.0);//-2 so that we can have Y-down
                        }
                        {
                            self.current_buffer = r_state.device.create_buffer_init(&BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(&vbuf),
                                usage: BufferUsages::VERTEX,
                            });
                        }
                        {
                            rpass_quad.set_vertex_buffer(0, self.current_buffer.slice(..));
                            rpass_quad.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                            rpass_quad.draw_indexed(0..6, 0, 0..1);
                        }
                    }
                    DrawNode2D::Text(text) => {
                        self.debug_glyph_brush.queue(Section {
                            screen_position: (text.transform.abs_pos.x, text.transform.abs_pos.y),
                            bounds: (r_state.get_width() as f32, r_state.get_height() as f32),
                            text: vec![Text::new(text.text.as_str())
                                .with_color(text.color.to_array())
                                .with_scale(text.scale)],
                            ..Section::default()
                        });
                        /*self.debug_glyph_brush
                        .draw_queued(
                            &r_state.device,
                            &mut self.debug_glyph_staging_belt,
                            encoder,
                            view,
                            window_size.x as u32,
                            window_size.y as u32,
                        );*/
                    }
                }
            }
        }

        /*self.debug_glyph_brush.queue(Section {
            screen_position: (30.0, 30.0),
            bounds: (r_state.get_width() as f32, r_state.get_height() as f32),
            text: vec![Text::new("Hello world!")
                .with_color([1.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.debug_glyph_brush
        .draw_queued(
            &r_state.device,
            &mut self.debug_glyph_staging_belt,
            encoder,
            view,
            window_size.x as u32,
            window_size.y as u32,
        )
        .expect("Draw queued");
        self.debug_glyph_staging_belt.finish();*/
        //self.debug_glyph_staging_belt.recall();
    }

    fn draw_quad<'a>(&'a mut self, quad: &mut QuadDrawNode2D, r_state: &mut WgpuState, mut rpass: RenderPass<'a>) {
        let window_size = &r_state.get_size();
        let mut vbuf: [UIVertex; 4] = [
            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x, quad.quad.abs_pos.y, 1.0]},
            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x + quad.quad.abs_size.x, quad.quad.abs_pos.y, 1.0]},
            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x + quad.quad.abs_size.x, quad.quad.abs_pos.y + quad.quad.abs_size.y, 1.0]},
            UIVertex {uv: [0.0, 0.0], color: [quad.color.x, quad.color.y, quad.color.z, quad.color.w], pos: [quad.quad.abs_pos.x, quad.quad.abs_pos.y + quad.quad.abs_size.y, 1.0]}
        ];
        for mut ele in &mut vbuf {
            ele.pos[0] = (ele.pos[0] / (window_size.x as f32)) * 2.0 - 1.0;
            ele.pos[1] = -((ele.pos[1] / (window_size.y as f32)) * 2.0 - 1.0);//-2 so that we can have Y-down
        }
        self.current_buffer = r_state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vbuf),
            usage: BufferUsages::VERTEX,
        });
        rpass.set_vertex_buffer(0, self.current_buffer.slice(..));
        rpass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..6, 0, 0..1);
    }

    pub fn on_resized(&mut self, width: u32, height: u32, device: &Device) {
        //self.screen_size_bind_group = Pipeline2D::create_screen_size_bind_group(width, height, &Pipeline2D::get_screen_size_layout(device), device)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct UIVertex {
    uv: [f32; 2],
    color: [f32; 4],
    pos: [f32; 3]
}

impl UIVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x3];

    fn describe() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
        
    }
}