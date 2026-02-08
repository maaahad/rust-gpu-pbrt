use std::sync::Arc; 
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
}; 

// TODO: (maaahad) this should go to lib.rs module
pub struct State {
    surface: wgpu::Surface<'static>, 
    device: wgpu::Device, 
    queue: wgpu::Queue, 
    configs: wgpu::SurfaceConfiguration, 
    is_surface_configured: bool, 
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size(); 

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default() 
        }); 

        let surface = instance.create_surface(window.clone()).unwrap(); 

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(), 
                compatible_surface: Some(&surface), 
                force_fallback_adapter: false
            }
        ).await?; 

        // TODO: (maaahad) test with adapter_enumerate

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None, 
                required_features: wgpu::Features::empty(), 
                experimental_features: wgpu::ExperimentalFeatures::disabled(), 
                required_limits: wgpu::Limits::default(), 
                memory_hints: Default::default(), 
                trace: wgpu::Trace::Off 
            }
        ).await?; 

        let surface_caps = surface.get_capabilities(&adapter); 

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let configs = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
            format: surface_format, 
            width: size.width, 
            height: size.height, 
            present_mode: surface_caps.present_modes[0], 
            alpha_mode: surface_caps.alpha_modes[0], 
            view_formats: vec![], 
            desired_maximum_frame_latency: 2 
        }; 

        Ok(Self{window, surface, device, queue, configs, is_surface_configured: false})
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        println!("Resize : window resize"); 
    }

    pub fn render(&mut self) {
        self.window.request_redraw(); 
    }
}

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new(_event_loop: &EventLoop<State>) -> Self {
        Self {
            state: None, 
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes(); 
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(window)).unwrap()); 
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        // wasm stuff
        self.state = Some(event); 
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _windowId: WindowId, event:WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas, 
            None => return, 
        }; 

        match event {
            WindowEvent::CloseRequested => event_loop.exit(), 
            WindowEvent::Resized(size) => state.resize(size.width, size.height), 
            WindowEvent::RedrawRequested => state.render(), 
            _ => println!("Add KeyboardInput"), 
            // Window::KeyboardInput{..} => pringln!("Keyboard Input"), 
        }

    }

}

pub fn run() -> anyhow::Result<()> {
    env_logger::init(); 

    let event_loop = EventLoop::with_user_event().build()?; 

    let mut app = App::new(&event_loop); 

    event_loop.run_app(&mut app)?; 

    Ok(())
}
fn main() {
    run(); 
}
