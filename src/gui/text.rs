use crate::gui::{GuiContext, Container, Widget, WidgetType, Offset};
use crate::utils::{
    console::RawConsole,
    doryen::{Color, TextAlign as RawTextAlign},
};

pub enum TextAlign {
    Left,
    Right,
    Center,
}

pub struct Label {
    container: Option<Container>,
    text: String,
    fore_color: Color,
    back_color: Color,
    align: TextAlign,
}

impl Label {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Label {
            container: None,
            text: text.into(),
            fore_color: (255, 255, 255, 255),
            back_color: (0, 0, 0, 255),
            align: TextAlign::Left,
        }
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn fore_color(mut self, color: Color) -> Self {
        self.fore_color = color;
        self
    }

    pub fn back_color(mut self, color: Color) -> Self {
        self.back_color = color;
        self
    }
}

impl Widget for Label {
    fn render(&self, console: &mut RawConsole, ctx: &GuiContext, offset: Offset) {
        let container = self.to_container(ctx.get_screen_size());        

        console.print(
            (container.x + offset.x) as i32, 
            (container.y + offset.y) as i32, 
            &self.text, 
            RawTextAlign::Left, 
            Some(self.fore_color), 
            Some(self.back_color),
        );
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Static
    }

    fn set_container(&mut self, container: Container) {
        self.container = Some(container);
    }

    fn to_container(&self, screen: Container) -> Container {
        let h = 1;
        let w = self.text.len() as u32;
        let (x, y) = match self.container {
            Some(cont) => match self.align {
                TextAlign::Left => (cont.x, cont.y),
                TextAlign::Right => (cont.x + (cont.w - w), 0),
                TextAlign::Center => (cont.x + ((cont.w - w) / 2), 0),
            },
            None => match self.align {
                TextAlign::Left => (0, 0),
                TextAlign::Right => (screen.w - w, 0),
                TextAlign::Center => ((screen.w - w) / 2, 0),
            }
        };

        Container { x, y, w, h }
    }
}