pub mod size;
pub mod containers;
pub mod button;
pub mod text;

pub use containers::*;
pub use size::*;
pub use button::*;
pub use text::*;

use std::sync::Arc;
use parking_lot::RwLock;
use sonja::prelude::*;

use crate::utils::console::RawConsole;

pub trait Widget: Send + Sync + 'static {
    fn render(&self, console: &mut RawConsole, ctx: &GuiContext, offset: Offset);

    fn widget_type(&self) -> WidgetType;

    fn to_container(&self, screen: Container) -> Container;

    fn local_ui(&self) -> Option<&Ui> {
        None
    }

    #[allow(unused_variables)]
    fn set_container(&mut self, container: Container) {}
}

pub enum WidgetType {
    Static,
    Interactive,
}

#[derive(Default)]
pub enum Layout {
    #[default]
    Absolute,
    Horizontal,
    Vertical,
}

#[derive(Default)]
pub struct Ui {
    layout: Layout,
    registry: Vec<Arc<RwLock<dyn Widget>>>,
}

impl Ui {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_horizontal() -> Self {
        Ui {
            layout: Layout::Horizontal,
            registry: vec![],
        }
    }

    pub fn new_vertical() -> Self {
        Ui {
            layout: Layout::Vertical,
            registry: vec![],
        }
    }

    pub(crate) fn set_container(&mut self, container: Container){
        for widget in &mut self.registry {
            widget.write().set_container(container)
        }
    }

    pub(crate) fn render(&self, console: &mut RawConsole, ctx: &GuiContext) {
        let screen = ctx.get_screen_size();
        let mut offset = Offset::default();

        for w in &self.registry {
            let widget = w.read();
            let widget_container = widget.to_container(screen);

            widget.render(console, ctx, offset);

            match self.layout {
                Layout::Horizontal => offset.x += widget_container.w,
                Layout::Vertical => offset.y += widget_container.h,
                _ => {},
            }

            if let Some(ui) = widget.local_ui() {
                ui.render(console, ctx);
            }
        }
    }

    pub(crate) fn clear(&mut self){
        self.registry.clear();
    }
}

#[derive(Default)]
pub struct GuiContext {
    pub(crate) ui: Ui,
    pub(crate) screen_size: (u32, u32),
}

impl GuiContext {
    pub fn get_screen_size(&self) -> Container {
        Container {
            x: 0,
            y: 0,
            w: self.screen_size.0,
            h: self.screen_size.1,
        }
    }

    pub(crate) fn render(&self, console: &mut RawConsole) {
        self.ui.render(console, self);
    }
}

impl Ui {
    pub fn add<W: Widget>(&mut self, widget: W) {
        self.registry.push(Arc::new(RwLock::new(widget)));
    }
}

pub fn render_gui(
    mut console: Write<RawConsole>,
    ctx: Read<GuiContext>,
){
    ctx.render(&mut console);
}
