use std::{borrow::Cow, rc::Rc};
use cgmath::Vector2;
use wgpu::{Device, ShaderModule, ColorTargetState, SurfaceConfiguration, Queue, Surface, TextureFormat, Features};
use winit::{
    window::{Window, WindowBuilder}, event_loop::EventLoop, dpi::PhysicalSize}
;

pub const DEFAULT_SCREEN_WIDTH: u32 = 1366;
pub const DEFAULT_SCREEN_HEIGHT: u32 = 768;

pub struct WgpuState {
    pub window: Window,
    pub surface: Surface,
    pub swapchain_format: TextureFormat,
    pub device: Device,
    pub config: SurfaceConfiguration,
    pub queue: Queue,

    pub shader_modules: ShaderModules,
    pub color_target: ColorTargetState,
}

impl WgpuState {
    pub async unsafe fn new(window: Window) -> WgpuState {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(&window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
    
        let mut features = wgpu::Features::default();
        features.set(Features::TEXTURE_COMPRESSION_BC, true);
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_defaults(),
                        //.using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");
    
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        //let swapchain_format = TextureFormat::
        let color_target: ColorTargetState = swapchain_format.into();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            //alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        // Load the shaders from disk
        let shader_modules = ShaderModules::new(&device);
         
        surface.configure(&device, &config);

        let r_state = WgpuState {
            window,
            surface,
            swapchain_format,
            device,
            config,
            queue,
            shader_modules,
            color_target,
        };
        return r_state;
    }

    pub fn get_width(&mut self) -> f32 {
        return self.window.inner_size().width as f32;
    }

    pub fn get_height(&mut self) -> f32 {
        return self.window.inner_size().height as f32;
    }

    pub fn get_size(&mut self) -> Vector2<f32> {
        let size = self.window.inner_size();
        return Vector2::new(size.width as f32, size.height as f32);
    }
}

pub struct ShaderModules {
    pub ui_shader: ShaderModule
}

impl ShaderModules {
    pub fn new(device: &Device) -> ShaderModules {
        let create_shader = |wgsl_source: &str, label: &str| {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(wgsl_source)),
            });
            return shader;
        };

        return ShaderModules {
            ui_shader: create_shader(include_str!("ui.wgsl"), "UI Shader"),
        }
    }
}