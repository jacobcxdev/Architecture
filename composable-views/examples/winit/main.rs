#![allow(dead_code)]
#![allow(unused_variables)]

use composable::Store;
use futures::executor::block_on;
use rfd::{MessageDialog, MessageLevel};
use std::collections::BTreeMap;
use std::sync::Arc;
use window::Action;

use window::menubar::Menu;
use window::{
    ActiveEventLoop, ApplicationHandler, ControlFlow, DeviceEvents, EventLoopError, EventLoopProxy,
    StartCause, Window, WindowEvent, WindowId,
};

mod gpu;
mod ink;
mod script;
mod settings;
mod window;

struct State {
    stores: BTreeMap<WindowId, (Store<script::State>, Arc<Window>)>,

    proxy: EventLoopProxy,
    menubar: Menu,
}

impl State {
    fn send(&mut self, window: WindowId, action: script::Action) {
        self.stores
            .entry(window)
            .and_modify(|store| store.0.send(action));
    }

    fn sync(&mut self, window: WindowId, action: script::Action) {
        self.stores
            .entry(window)
            .and_modify(|store| store.0.sync(action));
    }

    fn front_window(&self) -> Option<&Window> {
        self.stores
            .values()
            .map(|tuple| &*tuple.1)
            .find(|window| window.has_focus())
    }

    fn open_file(&mut self, window: WindowId) {}
}

impl ApplicationHandler<Action> for State {
    fn new_events(&mut self, active: &ActiveEventLoop, cause: StartCause) {
        if cause == StartCause::Init {
            self.user_event(active, Action::NewWindow);
        }
    }

    fn resumed(&mut self, active: &ActiveEventLoop) {}

    fn user_event(&mut self, active: &ActiveEventLoop, event: Action) {
        match event {
            Action::NewWindow => {
                let window = Arc::new(window::build(active));
                window::menubar::attach(&self.menubar, &window);

                let id = window.id();
                let proxy = self.proxy.clone();

                debug_assert_eq!(std::thread::current().name(), Some("main"));
                let wgpu = block_on(gpu::Surface::new(window.clone())); // must be on main thread
                let parent = window.clone();

                let state = script::State::new(wgpu, proxy.clone(), id);
                let store =
                    Store::with_dependency(|| state, move || window::dialogs(&parent, proxy));

                self.stores.insert(id, (store, window.clone()));
                self.send(id, script::Action::DefaultSize);
                self.send(id, script::Action::OpenFile);
            }
            Action::ErrorDialog(description, id) => {
                if let Some((_, window)) = self.stores.get(&id) {
                    let _ = MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Could not open file")
                        .set_description(description)
                        .set_parent(window)
                        .show();

                    self.window_event(active, id, WindowEvent::CloseRequested);
                }
            }
            Action::CloseWindow(id) => self.window_event(active, id, WindowEvent::CloseRequested),
            Action::DefaultSize => {
                self.front_window()
                    .map(|win| win.id())
                    .map(|id| self.send(id, script::Action::DefaultSize));
            }
        }
    }

    fn window_event(&mut self, active: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => self.send(id, script::Action::Redraw),
            WindowEvent::Resized(size) => {
                let (width, height) = size.into();
                let resize = script::Action::Resize { width, height };
                self.send(id, resize);
            }
            WindowEvent::CloseRequested => {
                self.stores.remove(&id);
                if self.stores.is_empty() {
                    active.exit();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    let (menubar, event_loop) = window::menubar::build()?;

    event_loop.listen_device_events(DeviceEvents::Never);
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();

    let mut state = State {
        stores: BTreeMap::default(),
        menubar,
        proxy,
    };

    event_loop.run_app(&mut state)
}
