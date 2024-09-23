use async_trait::async_trait;
use rfd::AsyncFileDialog;

use std::path::PathBuf;
use std::sync::{Arc, Weak};

pub use winit::application::ApplicationHandler;
pub use winit::dpi::{LogicalSize, Position};
pub use winit::error::EventLoopError;
pub use winit::event::{StartCause, WindowEvent};
pub use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents};
pub use winit::window::{Window, WindowId};

use winit::window::Theme;

pub type EventLoopProxy = winit::event_loop::EventLoopProxy<Action>;

pub mod menubar;

#[derive(Clone, Debug)]
pub enum Action {
    NewWindow,
    CloseWindow(WindowId),
    ErrorDialog(String, WindowId),
    DefaultSize,
}

pub fn build(active: &ActiveEventLoop) -> Window {
    let attributes = Window::default_attributes()
        .with_title("")
        .with_theme(Some(Theme::Light)) // None → current
        .with_position(Position::Logical(Default::default()))
        .with_visible(false);

    let window = active.create_window(attributes).expect("create_window");

    window
}

#[async_trait]
pub trait Dialogs {
    async fn open(&self) -> Option<PathBuf>;
    fn close(&self);

    fn resize(&self, width: u32, height: u32);
}

struct PlatformDialogs {
    window: Weak<Window>,
    proxy: EventLoopProxy,
}

#[async_trait]
impl Dialogs for PlatformDialogs {
    async fn open(&self) -> Option<PathBuf> {
        match self.window.upgrade() {
            None => None,
            Some(window) => {
                window.set_visible(true); // delay til now to ensure that the resize has finished…
                Some(
                    AsyncFileDialog::new()
                        .add_filter("Fountain", &["fountain", "spmd"])
                        .add_filter("Inkle", &["ink"])
                        .add_filter("Markdown", &["md"])
                        .set_parent(&window)
                        .pick_file()
                        .await?
                        .path()
                        .into(),
                )

                // TODO: retry sync again…
            }
        }
    }

    fn close(&self) {
        match self.window.upgrade() {
            None => {}
            Some(window) => self
                .proxy
                .send_event(Action::CloseWindow(window.id()))
                .unwrap(),
        }
    }

    fn resize(&self, width: u32, height: u32) {
        if let Some(window) = self.window.upgrade() {
            let _ = window.request_inner_size(LogicalSize::new(width, height));
            window.set_min_inner_size(Some(LogicalSize::new(width, 256)));
        }
    }
}

pub fn dialogs(window: &Arc<Window>, event_loop_proxy: EventLoopProxy) -> Box<dyn Dialogs> {
    Box::new(PlatformDialogs {
        window: Arc::downgrade(window),
        proxy: event_loop_proxy,
    })
}
