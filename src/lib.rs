use flatbox::prelude::*;

pub mod game;
pub mod gui;
pub mod utils;
pub mod prelude; 

use crate::utils::{
    WindowBuilderWrapper,
    doryen::{App, AppOptions, Engine, DoryenApi, UpdateEvent},
    console::RawConsole,
};

use crate::gui::GuiContext;

pub trait DoryenRenderSystemExt {
    fn add_render_system<Args, Ret, S>(&mut self, system: S) -> &mut Self 
    where
        S: 'static + System<Args, Ret> + Send;

    fn flush_render_systems(&mut self) -> &mut Self;
}

impl DoryenRenderSystemExt for Flatbox { 
    fn add_render_system<Args, Ret, S>(&mut self, system: S) -> &mut Self 
        where
            S: 'static + System<Args, Ret> + Send 
    {
        self.schedules.get_mut("render").unwrap().add_system(system);
        self
    }

    fn flush_render_systems(&mut self) -> &mut Self {
        self.schedules.get_mut("render").unwrap().flush();
        self
    }
}

struct FlatboxDoryen {
    flatbox: Flatbox,
    raw_console: RawConsole,
    gui_context: GuiContext,
}

impl FlatboxDoryen {
    fn interchange_console(&mut self, api: &mut dyn DoryenApi) {
        std::mem::swap(api.con(), self.raw_console.0.as_mut().unwrap());
    }

    fn is_exit(&self) -> bool {
        match self.flatbox.events.get_handler::<AppExit>().unwrap().read() {
            Some(_) => true,
            _ => false,
        }
    }
}

impl Engine for FlatboxDoryen {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.gui_context.screen_size = api.con().get_size();

        let mut setup_systems = self.flatbox.schedules.get_mut("setup").unwrap().build();

        self.interchange_console(api);
        setup_systems.execute((
            &mut self.flatbox.world,
            &mut self.flatbox.events,
            &mut self.flatbox.time_handler,
            &mut self.flatbox.physics_handler,
            &mut self.flatbox.asset_manager,
            &mut self.raw_console,
            &mut self.gui_context,
        )).expect("Cannot execute setup schedule");
        self.interchange_console(api);
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let mut update_systems = self.flatbox.schedules.get_mut("update").unwrap().build();

        self.interchange_console(api);
        update_systems.execute((
            &mut self.flatbox.world,
            &mut self.flatbox.events,
            &mut self.flatbox.time_handler,
            &mut self.flatbox.physics_handler,
            &mut self.flatbox.asset_manager,
            &mut self.raw_console,
            &mut self.gui_context,
        )).expect("Cannot execute update schedule");
        self.interchange_console(api);
        
        if self.is_exit() {
            Some(UpdateEvent::Exit)
        } else {
            None
        }
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        let mut render_systems = self.flatbox.schedules.get_mut("render").unwrap()
            .add_system(gui::render_gui)
            .build();

        self.interchange_console(api);
        render_systems.execute((
            &mut self.flatbox.world,
            &mut self.flatbox.events,
            &mut self.flatbox.time_handler,
            &mut self.flatbox.physics_handler,
            &mut self.flatbox.asset_manager,
            &mut self.raw_console,
            &mut self.gui_context,
        )).expect("Cannot execute update schedule");
        self.interchange_console(api);

        self.gui_context.ui.clear();
    }

    fn resize(&mut self, api: &mut dyn DoryenApi){
        self.gui_context.screen_size = api.get_screen_size();
    }
}

pub struct DoryenExtension;

impl Extension for DoryenExtension {
    fn apply(&self, app: &mut Flatbox) {
        app
            .add_events::<AppExit>()
            .set_runner(Box::new(flatbox_doryen_run))
            .schedules.insert("render", Schedule::builder());
    }
}

fn flatbox_doryen_run(s: &mut Flatbox){
    let mut flatbox = empty_flatbox();
    std::mem::swap(s, &mut flatbox);

    let mut app = App::new(AppOptions::from(
        WindowBuilderWrapper {
            inner: flatbox.window_builder.clone()
        }
    ));

    app.set_engine(Box::new(FlatboxDoryen { 
        flatbox, 
        raw_console: RawConsole::default(),
        gui_context: GuiContext::default(),
    }));
    app.run();
}

fn empty_flatbox() -> Flatbox {
    Flatbox::init(WindowBuilder {
        init_logger: false, 
        ..Default::default()
    })
}