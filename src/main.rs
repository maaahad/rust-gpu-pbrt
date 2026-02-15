extern crate glfw; 
use glfw::{fail_on_errors, Action, Context, Key }; 

fn main() {

    let mut glfw = glfw::init(fail_on_errors!()).unwrap(); 

    let (mut window, event) = glfw.create_window(400, 400, "GLFW Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window"); 

    window.make_current(); 
    window.set_key_polling(true); 

    while !window.should_close() {
        window.swap_buffers(); 
        glfw.poll_events(); 
    }
}
