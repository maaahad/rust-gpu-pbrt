use std::sync::Arc; 
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
}; 

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self{window})
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
