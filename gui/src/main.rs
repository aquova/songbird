use imgui::*;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize {
        width: 160.0,
        height: 144.0,
    });
    window.set_title("Songbird");
    loop {
        // Do nothing
    }
}
