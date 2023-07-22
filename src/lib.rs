use std::ops::{Deref, DerefMut};

use sonja::prelude::*;
use doryen_rs::{AppOptions, Engine, DoryenApi, UpdateEvent, DEFAULT_CONSOLE_WIDTH, DEFAULT_CONSOLE_HEIGHT, App};

pub mod color;
pub mod input;
pub mod prelude; 

use crate::color::RegisterColors;

struct SonjaDoryen {
    pub sonja: Sonja,
}

impl SonjaDoryen {
    fn is_exit(&self) -> bool {
        match self.sonja.events.get_handler::<AppExit>().unwrap().read() {
            Some(_) => true,
            _ => false,
        }
    }
}

impl Engine for SonjaDoryen {
    fn init(&mut self, _api: &mut dyn DoryenApi) {
        let mut setup_systems = self.sonja.setup_systems.build();

        setup_systems.execute((
            &mut self.sonja.world,
            &mut self.sonja.events,
            &mut self.sonja.time_handler,
            &mut self.sonja.physics_handler,
            &mut self.sonja.asset_manager,
        )).expect("Cannot execute setup schedule");
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let mut systems = self.sonja.systems.build();

        systems.execute((
            &mut self.sonja.world,
            &mut self.sonja.events,
            &mut self.sonja.time_handler,
            &mut self.sonja.physics_handler,
            &mut self.sonja.asset_manager,
        )).expect("Cannot execute setup schedule");

        // Register colors
        let mut color_events = self.sonja.events.get_handler::<RegisterColors>().unwrap();
        if let Some(colors) = color_events.read() {
            for (name, value) in colors.colors {
                api.con().register_color(name, value);
            }
        }
        color_events.clear();
        
        // Check quit
        if self.is_exit() {
            Some(UpdateEvent::Exit)
        } else {
            None
        }
    }

    fn render(&mut self, _api: &mut dyn DoryenApi) {
    
    }
}

pub struct DoryenExtension;

impl Extension for DoryenExtension {
    fn apply(&self, app: &mut Sonja) {
        app
            .add_events::<RegisterColors>()
            .add_events::<AppExit>()

            .set_runner(Box::new(sonja_doryen_run));
    }
}

fn sonja_doryen_run(s: &mut Sonja){
    let mut sonja = Sonja::init(WindowBuilder {
        init_logger: Some(false), 
        ..Default::default()
    });
    std::mem::swap(s, &mut sonja);

    let mut app = App::new(AppOptions::from(
        WindowBuilderWrapper {
            inner: sonja.window_builder.clone()
        }
    ));

    app.set_engine(Box::new(SonjaDoryen { sonja }));
    app.run();
}

impl From<WindowBuilderWrapper> for AppOptions {
    fn from(source: WindowBuilderWrapper) -> Self {
        AppOptions {
            window_title: source.title.unwrap_or("Doryen game").to_owned(),
            fullscreen: source.fullscreen.unwrap_or(false),
            resizable: source.resizable.unwrap_or(true),
            
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