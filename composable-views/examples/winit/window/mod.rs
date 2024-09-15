pub use winit::application::ApplicationHandler;
pub use winit::dpi::{LogicalSize, Position, Size};
pub use winit::error::EventLoopError;
pub use winit::event::{StartCause, WindowEvent};
pub use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents};
pub use winit::window::{Window, WindowId};

use winit::window::Theme;

pub type EventLoopProxy = winit::event_loop::EventLoopProxy<Action>;

pub mod menubar;

pub const DEFAULT_SIZE: (f32, f32) = (1366.0, 1024.0);
pub const MIN_SIZE: (f32, f32) = (1024.0, 768.0);

#[derive(Clone, Debug)]
pub enum Action {
    NewWindow,
    DefaultSize,
    ErrorDialog(String, WindowId),
}

pub fn build(event_loop: &ActiveEventLoop) -> Window {
    #[allow(unused_mut)]
    let mut attributes = Window::default_attributes()
        .with_title("")
        .with_theme(Some(Theme::Light)) // None → current
        .with_position(Position::Logical(Default::default()))
        .with_inner_size(LogicalSize {
            width: DEFAULT_SIZE.0,
            height: DEFAULT_SIZE.1,
        })
        .with_min_inner_size(LogicalSize {
            width: MIN_SIZE.0,
            height: MIN_SIZE.1,
        });

    let window = event_loop.create_window(attributes).expect("create_window");

    window
}
