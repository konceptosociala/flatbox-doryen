pub use doryen_rs as doryen;

pub mod console;
pub mod input;

use std::ops::{Deref, DerefMut};
use sonja::WindowBuilder;

use doryen::{AppOptions, DEFAULT_CONSOLE_WIDTH, DEFAULT_CONSOLE_HEIGHT};

#[derive(Default, Clone, Debug)]
pub struct WindowBuilderWrapper {
    pub inner: WindowBuilder,
}

impl WindowBuilderWrapper {
    pub fn new() -> Self {
        WindowBuilderWrapper::default()
    }
}

impl From<WindowBuilder> for WindowBuilderWrapper {
    fn from(source: WindowBuilder) -> Self {
        WindowBuilderWrapper { inner: source }
    }
}

impl Deref for WindowBuilderWrapper {
    type Target = WindowBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for WindowBuilderWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<WindowBuilderWrapper> for AppOptions {
    fn from(source: WindowBuilderWrapper) -> Self {
        AppOptions {
            window_title: source.title.to_owned(),
            fullscreen: source.fullscreen,
            resizable: source.resizable,
            
            console_width: DEFAULT_CONSOLE_WIDTH,
            console_height: DEFAULT_CONSOLE_HEIGHT,
            screen_width: DEFAULT_CONSOLE_WIDTH * 8,
            screen_height: DEFAULT_CONSOLE_HEIGHT * 8,
            font_path: "Aesomatica_16x16.png".to_owned(),
            vsync: true,
            show_cursor: false,
            intercept_close_request: false,
            max_fps: 0,
        }
    }
}